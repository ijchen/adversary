use std::panic::AssertUnwindSafe;

use crate::report::{FailureCause, TestOutcome};

use super::Test;

impl<T> Test<T> for fn(T) {
    fn test(&self, value: T) -> TestOutcome {
        match chillpill::catch(AssertUnwindSafe(|| self(value))) {
            Ok(()) => TestOutcome::Passed,
            Err(panic_data) => TestOutcome::Failed {
                cause: FailureCause::ExpectedPanic {
                    panic_data: panic_data.into(),
                },
            },
        }
    }
}
