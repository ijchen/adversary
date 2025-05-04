use crate::report::Report;

pub enum TestResult<T> {
    Passed,
    Failed(Report<T>),
    InvalidConfig(String),
}

impl<T> TestResult<T> {
    pub fn passed(&self) -> bool {
        matches!(self, TestResult::Passed)
    }

    pub fn unwrap_report(self) -> Report<T> {
        match self {
            TestResult::Failed(report) => report,
            TestResult::Passed => {
                panic!("called `TestResult::unwrap_report()` on a `Passed` value")
            }
            TestResult::InvalidConfig(_) => {
                panic!("called `TestResult::unwrap_report()` on an `InvalidConfig` value")
            }
        }
    }
}
