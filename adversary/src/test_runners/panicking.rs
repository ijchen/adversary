use std::{
    any::Any,
    cell::{Cell, RefCell},
    panic::{PanicHookInfo, RefUnwindSafe, UnwindSafe},
    sync::{Arc, LazyLock, Mutex},
};

use crate::{
    rand::Rng,
    report::{PanicInfo, PanicLocation, Report},
    IntoValueGen,
};

use super::run_test;

fn extract_panic_message(any: Box<dyn Any + Send + 'static>) -> Option<String> {
    // Try downcasting to a &str first
    let any = match any.downcast::<&str>() {
        Ok(s) => return Some(s.to_string()),
        Err(any) => any,
    };

    // Downcasting to a &str failed, try downcasting to a String
    any.downcast::<String>().ok().map(|s| *s)
}

// TODO: document how this interacts with the panic hook (no other code besides
// other calls to this function may set the panic hook for the entire duration
// of any call to this function, or arbitrarily weird things (but not Undefined
// Behavior) may happen with the panic hook)
pub fn run_test_panics<T: UnwindSafe>(
    test: impl Fn(T) + RefUnwindSafe,
    generator: impl IntoValueGen<T>,
    rng: &mut impl Rng,
) -> Result<(), Box<Report<T>>> {
    enum PanicHookStatus {
        NotReplaced,
        Replaced {
            old_hook: Box<dyn Fn(&PanicHookInfo<'_>) + 'static + Sync + Send>,
            active_testing_threads: usize,
        },
    }
    thread_local! {
        /// Whether or not this thread is currently running an adversary panic
        /// based test.
        static THREAD_PANIC_TESTING: Cell<bool> = const { Cell::new(false) };

        // NOTE(ichen): these thread-locals allow us to smuggle out the panic
        // location and message from the panic hook and catch_unwind closure,
        // respectively, without concurrent threads potentially clobbering each
        // other's data.
        static PANIC_LOCATION: RefCell<Option<PanicLocation>> = const { RefCell::new(None) };
        static PANIC_MESSAGE: RefCell<Option<String>> = const { RefCell::new(None) };
    }

    /// Whether or not the panic hook is currently replaced, and if so how many
    /// threads are currently relying on the adversary panic hook as well as the
    /// old panic hook to delegate to in case any unrelated threads panic while
    /// we've hijacked the panic hook.
    ///
    /// This is necessary since the panic hook is a shared resource across all
    /// threads, and we want to handle potentially multiple concurrent adversary
    /// panic based tests gracefully.
    static PANIC_HOOK_STATUS: LazyLock<Arc<Mutex<PanicHookStatus>>> =
        LazyLock::new(|| Arc::new(Mutex::new(PanicHookStatus::NotReplaced)));

    // This thread is now in an adversary panic based test - if our custom panic
    // hook detects a panic from this thread, it should have our custom behavior
    THREAD_PANIC_TESTING.set(true);

    // Replace the panic hook with our custom one. If we've already replaced it,
    // increment the number of threads currently doing panic based testing.
    let mut panic_hook_status = PANIC_HOOK_STATUS.lock().unwrap();
    match *panic_hook_status {
        PanicHookStatus::NotReplaced => {
            let old_hook = std::panic::take_hook();
            *panic_hook_status = PanicHookStatus::Replaced {
                old_hook,
                active_testing_threads: 1,
            };

            std::panic::set_hook(Box::new(|panic_hook_info| {
                if THREAD_PANIC_TESTING.get() {
                    let panic_location = panic_hook_info.location().map(PanicLocation::from);

                    PANIC_LOCATION.set(panic_location);
                } else {
                    // We're currently running the replaced panic hook, so if
                    // PANIC_HOOK_STATUS isn't `Replaced`, that's a logic error
                    let PanicHookStatus::Replaced { old_hook, .. } =
                        &*PANIC_HOOK_STATUS.lock().unwrap()
                    else {
                        unreachable!()
                    };

                    // This panic wasn't from a thread doing adversary panic
                    // based testing - delegate to the old panic hook
                    old_hook(panic_hook_info);
                }
            }));
        }
        PanicHookStatus::Replaced {
            ref mut active_testing_threads,
            ..
        } => {
            *active_testing_threads = active_testing_threads.checked_add(1).unwrap();
        }
    }
    drop(panic_hook_status);

    // Actually run the test, letting `run_test` do the bulk of the work for us
    let test_result = run_test(
        |value| {
            std::panic::catch_unwind(|| test(value))
                .map_err(|e| PANIC_MESSAGE.set(extract_panic_message(e)))
                .is_ok()
        },
        generator,
        rng,
    );

    // This thread is no longer in an adversary panic based test - if our custom
    // panic hook detects a panic from this thread, it should delegate to the
    // old panic hook.
    THREAD_PANIC_TESTING.set(false);

    // Decrement the number of threads currently doing panic based testing,
    // restoring the original panic hook if we were the last one.
    let mut panic_hook_status = PANIC_HOOK_STATUS.lock().unwrap();
    match &mut *panic_hook_status {
        PanicHookStatus::NotReplaced => unreachable!(),
        PanicHookStatus::Replaced {
            active_testing_threads: 0,
            ..
        } => {
            let PanicHookStatus::Replaced {
                active_testing_threads: 0,
                old_hook,
            } = std::mem::replace(&mut *panic_hook_status, PanicHookStatus::NotReplaced)
            else {
                // The pattern in this `if let` will match anything that matches
                // the pattern for the match arm we're in, so entering this else
                // branch would indicate a logic error.
                unreachable!()
            };
            std::panic::set_hook(old_hook);
        }
        PanicHookStatus::Replaced {
            active_testing_threads: active_testing_threads @ 1..,
            ..
        } => {
            *active_testing_threads = active_testing_threads.checked_sub(1).unwrap();
        }
    }
    drop(panic_hook_status);

    // Extract out the panic message and location from our thread-locals, and
    // insert them into the report.
    test_result.map_err(|report| {
        Box::new(Report {
            panic_info: Some(PanicInfo {
                message: PANIC_MESSAGE.take(),
                location: PANIC_LOCATION.take(),
            }),
            ..*report
        })
    })
}
