use std::error::Error;

use super::PanicData;

#[derive(Debug)]
pub enum TestOutcome {
    Passed,
    Failed { cause: FailureCause },
}

impl TestOutcome {
    pub fn passed(&self) -> bool {
        match self {
            TestOutcome::Passed => true,
            TestOutcome::Failed { .. } => false,
        }
    }
}

#[derive(Debug)]
pub enum FailureCause {
    NormalFailure,
    UnexpectedPanic { panic_data: PanicData },
    ExpectedPanic { panic_data: PanicData },
    WrongPanicMessage { panic_data: PanicData },
    ResultErr { error: Box<dyn Error> },
}
