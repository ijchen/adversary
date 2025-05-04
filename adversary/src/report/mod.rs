mod observation;
mod panic;
pub mod renderer;
mod shrink_step;
mod test_outcome;

pub use observation::{Importance, Observation};
pub use panic::{PanicData, PanicLocation};
pub use renderer::ReportRenderer;
pub use shrink_step::ShrinkStep;
pub use test_outcome::{FailureCause, TestOutcome};

#[derive(Debug)]
pub struct Report<T> {
    /// The name of the failing test, if available.
    ///
    /// It's worth noting that this is not usually available - currently, it is
    /// only added by the [`adv_test`](crate::adv_test) macro.
    // TODO(ichen): allow a test configuration option provided to test runners
    // that allows the user to specify the test name - and update these docs
    pub test_name: Option<String>,

    /// The number of passing attempts before test failure. Zero indicates that
    /// the test immediately failed. [`u64::MAX`] indicates that the test failed
    /// [`u64::MAX`] *or more* times. In other words, test runs over this value
    /// will saturate. For what it's worth, at 5 billion test runs per second,
    /// it would take over 116 years to reach this limit.
    pub passing_runs: u64,

    /// Any [`Observation`]s made during shrinking
    pub observations: Vec<Observation>,

    /// Each step taken during the shrinking process, from the original failing
    /// value as the first element to the simplest failing value as the last
    /// element.
    ///
    /// For any normal shrinking process, it is guaranteed that the first step
    /// will be a failing non-informational step, since it is by definition the
    /// original failing value. This guarantee is upheld by test runners in this
    /// crate which generate [`Report`]s, although since the field is `pub`,
    /// nothing stops other code from violating this invariant by modifying the
    /// field directly.
    pub shrink_steps: Vec<ShrinkStep<T>>,
}

impl<T> Report<T> {
    // TODO(ichen): this should probably return `Option` instead of panicking
    // (same for other methods)
    /// Returns the original failing value.
    ///
    /// # Panics
    /// If element 0 in the `shrink_step` field doesn't exist, or doesn't have
    /// both `test_passed` and `just_informational` set to false. This shouldn't
    /// happen, because there should always at least be the original failing
    /// value, although if the `shrink_step` field has been modified by the user
    /// this may occur.
    pub fn original_failing_value(&self) -> &T {
        let first_shrinking_step = &self
            .shrink_steps
            .first()
            .expect("`Report`'s `shrink_steps` field was empty");

        assert!(
            !first_shrinking_step.just_informational,
            "first shrinking step was just informational"
        );
        assert!(
            !first_shrinking_step.outcome.passed(),
            "first shrinking step was not a test failure"
        );

        &first_shrinking_step.value
    }

    /// Returns the simplest failing value.
    ///
    /// # Panics
    /// If the `shrink_step` field does not contain any [`ShrinkStep`]s that
    /// had a failing outcome and `just_informational` set to false. This
    /// shouldn't happen, because there should always at least be the original
    /// failing value, although if the `shrink_step` field has been modified by
    /// the user this may occur.
    pub fn simplest_failing_value(&self) -> &T {
        &self
            .shrink_steps
            .iter()
            .rfind(|step| !step.outcome.passed() && !step.just_informational)
            .expect("there should always be at least one failing non-informational step, the original failing value")
            .value
    }

    /// Returns the cause of the simplest failing step.
    ///
    /// # Panics
    /// If the `shrink_step` field does not contain any [`ShrinkStep`]s that
    /// had a failing outcome and `just_informational` set to false. This
    /// shouldn't happen, because there should always at least be the original
    /// failing value, although if the `shrink_step` field has been modified by
    /// the user this may occur.
    pub fn simplest_failing_cause(&self) -> &FailureCause {
        self
            .shrink_steps
            .iter()
            .filter_map(|step| {
                match step.outcome {
                    TestOutcome::Passed => None,
                    TestOutcome::Failed { ref cause } => (!step.just_informational).then_some(cause),
                }
            })
            .next_back()
            .expect("there should always be at least one failing non-informational step, the original failing value")
    }

    /// Renders this report using the provided [`ReportRenderer`].
    pub fn render<R: ReportRenderer>(&self, converter: impl Fn(&T) -> R::Converted) -> R::Output {
        R::render(self, converter)
    }
}
