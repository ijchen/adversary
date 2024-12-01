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
    pub test_name: Option<String>,
    pub panic_info: Option<PanicInfo>,
    pub passing_runs: u64,
    pub observations: Vec<Observation>,
    // If no shrinking occurred, this will be empty. If some shrinking occurred,
    // the first elem is the original failing input, and the last elem is the
    // step right before the simplest failing input
    pub shrink_steps: Vec<ShrinkStep<T>>,
    pub simplest_failing_input: T,
}

impl<T> Report<T> {
    pub fn render<R: ReportRenderer>(&self) -> R::Output {
        R::render(self)
    }
}
