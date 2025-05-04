use std::panic::AssertUnwindSafe;

use crate::report::{FailureCause, TestOutcome};

use super::Test;

/// Returns a [`Test`] that passes if the function panics, and fails if it does
/// not.
pub fn should_panic<T>(test: fn(T)) -> impl Test<T> {
    ShouldPanicTest {
        test,
        expected_message: ExpectedPanicMessage::Any,
    }
}

/// Returns a [`Test`] that passes if the function panics with exactly the
/// provided expected message, and fails if it either panics with a different
/// message or does not panic at all.
pub fn should_panic_with_message<T>(test: fn(T), expected_message: String) -> impl Test<T> {
    ShouldPanicTest {
        test,
        expected_message: ExpectedPanicMessage::Exactly(expected_message),
    }
}

enum ExpectedPanicMessage {
    Any,
    Exactly(String),
}

struct ShouldPanicTest<T> {
    test: fn(T),
    expected_message: ExpectedPanicMessage,
}
impl<T> Test<T> for ShouldPanicTest<T> {
    fn test(&self, value: T) -> TestOutcome {
        match (
            chillpill::catch(AssertUnwindSafe(|| (self.test)(value))),
            &self.expected_message,
        ) {
            (Ok(()), _) => TestOutcome::Failed {
                cause: FailureCause::NormalFailure,
            },
            (Err(_), ExpectedPanicMessage::Any) => TestOutcome::Passed,
            (Err(panic_data), ExpectedPanicMessage::Exactly(expected)) => {
                if panic_data
                    .payload_as_string()
                    .is_some_and(|actual| expected == actual)
                {
                    TestOutcome::Passed
                } else {
                    TestOutcome::Failed {
                        cause: FailureCause::WrongPanicMessage {
                            panic_data: panic_data.into(),
                        },
                    }
                }
            }
        }
    }
}
