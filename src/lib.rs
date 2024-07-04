mod adversarial;
mod exhaustive;
pub mod generators;
mod input_generator;
mod report;
mod sample;
mod shrink;
mod test_runner;

pub use rand;

pub use adversarial::Adversarial;
pub use exhaustive::Exhaustive;
pub use input_generator::InputGenerator;
pub use report::Report;
pub use sample::Sample;
pub use shrink::Shrink;
pub use test_runner::{run_test, run_test_panics};

#[cfg(test)]
mod tests {
    use super::*;
    use generators::any;

    #[test]
    fn my_test_2() {
        const FAIL_THRESHOLD: u32 = 123454321;
        fn check(n: u32) -> bool {
            n < FAIL_THRESHOLD
        }

        let failing_input = run_test(|&n| check(n), any(), &mut rand::thread_rng());

        assert_eq!(
            failing_input.unwrap_err().minimal_failing_input,
            Some(FAIL_THRESHOLD)
        );
    }

    #[test]
    fn my_test_3() {
        const FAIL_THRESHOLD: u32 = 123454321;
        fn check(n: u32) {
            assert!(n < FAIL_THRESHOLD)
        }

        let failing_input = run_test_panics(|&n| check(n), any(), &mut rand::thread_rng());

        assert_eq!(
            failing_input.unwrap_err().minimal_failing_input,
            Some(FAIL_THRESHOLD)
        );
    }

    #[test]
    fn my_test_4() {
        run_test_panics(
            |value: &u8| {
                assert!(*value > 3 || *value <= 3);
            },
            any(),
            &mut rand::thread_rng(),
        )
        .unwrap()
    }
}
