use std::{
    any::Any,
    cell::OnceCell,
    panic::{RefUnwindSafe, UnwindSafe},
};

use crate::{
    rand::Rng,
    report::{PanicInfo, PanicLocation},
};

use crate::{IntoInputGenerator, Report};

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

pub fn run_test_panics<T: UnwindSafe>(
    test: impl Fn(T) + RefUnwindSafe,
    generator: impl IntoInputGenerator<T>,
    rng: &mut impl Rng,
) -> Result<(), Report<T>> {
    // TODO(ichen): see if we can avoid interior mutability hacks here
    let panic_message = OnceCell::new();
    let (tx, rx) = std::sync::mpsc::sync_channel::<Option<PanicLocation>>(1);

    // TODO(ichen): consider whether or not it's possible that we fail to
    // reinstate the panic hook after a panic somewhere between now and the end
    // of this function call... also surely this just completely trolls if
    // another thread running happens to panic at the same time as this running
    let old_panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_hook_info| {
        let panic_location = panic_hook_info.location().map(PanicLocation::from);
        tx.send(panic_location).unwrap_or(());
    }));

    let test_result = run_test(
        |value| {
            std::panic::catch_unwind(|| test(value))
                .map_err(|e| panic_message.set(extract_panic_message(e)).unwrap())
                .is_ok()
        },
        generator,
        rng,
    )
    .map_err(|report| Report {
        panic_info: Some(PanicInfo {
            message: panic_message.into_inner().flatten(),
            location: rx.try_recv().ok().flatten(),
        }),
        ..report
    });

    std::panic::set_hook(old_panic_hook);

    test_result
}
