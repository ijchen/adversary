mod observation;
mod panic_info;
pub mod renderer;
mod shrink_step;

pub use observation::{Importance, Observation};
pub use panic_info::{PanicInfo, PanicLocation};
pub use renderer::ReportRenderer;
pub use shrink_step::ShrinkStep;

#[derive(Debug, Clone)]
pub struct Report<T> {
    /// The name of the test, or [`None`] if the test name is unavailable.
    ///
    /// Note that the test name is typically unavailable - the main exception is
    /// the [`adv_test`](crate::adv_test) macro, which is able to insert the
    /// test name by virtue of being an all-powerful procedural macro.
    // TODO(ichen): allow passing the test name to the runners in a config arg
    pub test_name: Option<String>,

    /// The [`PanicInfo`] for the panic causing this test failure, if available.
    ///
    /// Note that this may be [`None`] if the test failure wasn't caused by a
    /// panic, or if the panic information was unavailable.
    pub panic_info: Option<PanicInfo>,

    /// The number of passing runs before test failure.
    ///
    /// 0 indicates the test failed immediately. [`u64::MAX`] indicates that the
    /// test failed [`u64::MAX`] *or more* times - in other words, over
    /// this value will saturate at [`u64::MAX`]. For what it's worth, at 5
    /// billion test runs per second, it would take over 116 years to reach this
    /// limit.
    pub passing_runs: u64,

    /// Any [`Observation`]s made by the test runner or shrinkers.
    pub observations: Vec<Observation>,

    /// Each step taken during the shrinking process, from the original failing
    /// input as the first element to the simplest failing input as the last
    /// element.
    ///
    /// For any normal shrinking process, it is guaranteed that the first step
    /// will be a failing non-informational step, since it is by definition the
    /// original failing input. This guarantee is upheld by test runners in this
    /// crate which generate [`Report`]s, although since the field is `pub`,
    /// nothing stops other code from violating this invariant by modifying the
    /// field directly.
    pub shrink_steps: Vec<ShrinkStep<T>>,
}

impl<T> Report<T> {
    /// Renders this report using the provided [`ReportRenderer`].
    pub fn render<R: ReportRenderer>(&self, converter: impl Fn(&T) -> R::Converted) -> R::Output {
        R::render(self, converter)
    }

    pub fn original_failing_input(&self) -> &T {
        &self.shrink_steps.first().expect("expected the `shrink_steps` field to have at least one failing non-informational step - the original failing input").value
    }

    pub fn simplest_failing_input(&self) -> &T {
        &self.shrink_steps
            .iter()
            .rfind(|step| !step.test_passed && !step.just_informational)
            .expect("expected the `shrink_steps` field to have at least one failing non-informational step - the original failing input")
            .value
    }
}
