mod bool;
mod panic;
mod result;
mod should_panic;

use crate::report::TestOutcome;

pub use should_panic::{should_panic, should_panic_with_message};

pub trait Test<T> {
    fn test(&self, value: T) -> TestOutcome;
}
