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
    use std::collections::HashSet;

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
            *failing_input.unwrap_err().minimal_failing_input(),
            FAIL_THRESHOLD
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
            *failing_input.unwrap_err().minimal_failing_input(),
            FAIL_THRESHOLD
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

    #[test]
    fn my_test_5() {
        fn add(a: u32, b: u32) -> u64 {
            u64::from(a) + u64::from(b)
        }

        // Zero identity
        run_test(
            |value: &_| add(*value, 0) == (*value).into(),
            any(),
            &mut rand::thread_rng(),
        )
        .unwrap();

        // Commutativity
        run_test(
            |value: &(_, _, ())| add(value.0, value.1) == add(value.1, value.0),
            any(),
            &mut rand::thread_rng(),
        )
        .unwrap();

        // Associativity
        run_test(
            |value: &(_, _, _)| match (
                add(value.1, value.2).try_into(),
                add(value.0, value.1).try_into(),
            ) {
                (Ok(sum1), Ok(sum2)) => add(value.0, sum1) == add(sum2, value.2),
                _ => true,
            },
            any(),
            &mut rand::thread_rng(),
        )
        .unwrap();
    }

    #[test]
    fn my_test_6() {
        assert_eq!(
            *run_test(
                |value: &Option<()>| value.is_some(),
                any(),
                &mut rand::thread_rng(),
            )
            .unwrap_err()
            .minimal_failing_input(),
            None
        )
    }

    #[test]
    fn my_test_7() {
        assert_eq!(
            *run_test(
                |value: &Option<u32>| value.is_none() || value.is_some_and(|n| n < 103),
                any(),
                &mut rand::thread_rng(),
            )
            .unwrap_err()
            .minimal_failing_input(),
            Some(103)
        )
    }

    #[test]
    fn my_test_8() {
        fn f(value: u8) -> String {
            format!("0x{value:02x}")
        }

        let v: HashSet<_> = any::<u8>().adv_map(f).exhaustive().collect();

        assert_eq!(v, (0..=u8::MAX).map(f).collect())
    }
}
