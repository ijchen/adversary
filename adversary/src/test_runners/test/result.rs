use std::{error::Error, panic::AssertUnwindSafe};

use crate::report::{FailureCause, TestOutcome};

use super::Test;

impl<T, E: Error + 'static> Test<T> for fn(T) -> Result<(), E> {
    fn test(&self, value: T) -> TestOutcome {
        match chillpill::catch(AssertUnwindSafe(|| self(value))) {
            Ok(Ok(())) => TestOutcome::Passed,
            Ok(Err(err)) => TestOutcome::Failed {
                cause: FailureCause::ResultErr {
                    error: Box::new(err),
                },
            },
            Err(panic_data) => TestOutcome::Failed {
                cause: FailureCause::UnexpectedPanic {
                    panic_data: panic_data.into(),
                },
            },
        }
    }
}
