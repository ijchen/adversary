pub mod generators;
mod input_generator;
mod input_generator_ext;
mod into_input_generator;
pub mod prelude;
mod report;
mod test_runner;

pub use rand;

pub use generators::{any, Canonical};
pub use input_generator::InputGenerator;
pub use input_generator_ext::InputGeneratorExt;
pub use into_input_generator::IntoInputGenerator;
pub use report::{Report, ShrinkStep};
pub use test_runner::{run_test, run_test_panics};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let report = run_test(|v: &bool| !v, any(), &mut crate::rand::thread_rng()).unwrap_err();
        assert_eq!(report.passing_runs, 1);
        assert_eq!(
            report.shrink_steps,
            vec![ShrinkStep::new(false, false, true)]
        );
        assert_eq!(report.simplest_failing_input, true);

        let report = run_test(|v: &bool| *v, any(), &mut crate::rand::thread_rng()).unwrap_err();
        assert_eq!(report.passing_runs, 0);
        assert_eq!(report.shrink_steps, vec![ShrinkStep::new(true, true, true)]);
        assert_eq!(report.simplest_failing_input, false);

        let report = run_test(|_: &bool| false, any(), &mut crate::rand::thread_rng()).unwrap_err();
        assert_eq!(report.passing_runs, 0);
        assert_eq!(
            report.shrink_steps,
            vec![ShrinkStep::new(true, true, false)]
        );
        assert_eq!(report.simplest_failing_input, false);

        run_test(|_: &bool| true, any(), &mut crate::rand::thread_rng()).unwrap();
    }
}
