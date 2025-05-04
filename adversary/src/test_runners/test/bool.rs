use std::panic::AssertUnwindSafe;

use crate::report::{FailureCause, TestOutcome};

use super::Test;

impl<T> Test<T> for fn(T) -> bool {
    fn test(&self, value: T) -> TestOutcome {
        match chillpill::catch(AssertUnwindSafe(|| self(value))) {
            Ok(true) => TestOutcome::Passed,
            Ok(false) => TestOutcome::Failed {
                cause: FailureCause::NormalFailure,
            },
            Err(panic_data) => TestOutcome::Failed {
                cause: FailureCause::UnexpectedPanic {
                    panic_data: panic_data.into(),
                },
            },
        }
    }
}
