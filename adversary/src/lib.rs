pub mod adapters;
mod chance;
pub mod generators;
pub mod prelude;
pub mod report;
mod sample;
mod self_test_helpers;
mod shrinker;
pub mod test_runners;
mod value_gen;

#[cfg(feature = "macros")]
pub use adversary_macros::adv_test;

// TODO(ichen): I don't really want to re-export this whole crate - we only need
// rand::Rng for ValueGen. Instead, have our own Rng type/trait (or maybe we get
// std::random?).
pub use rand;

pub use chance::Chance;
pub use generators::{Canonical, any, bool, just, just_with, length_gen, vec};
pub use value_gen::{IntoValueGen, RangeAwareValueGen, ValueGen, ValueGenExt};

// This message was brought to you from the Space Needle! 🛸🛸🛸

#[cfg(test)]
mod tests {
    use crate::{
        report::{FailureCause, ShrinkStep, TestOutcome},
        test_runners::{TestConfig, run_test_bool, run_test_panic},
    };

    use super::*;

    #[test]
    fn tmp_test() {
        fn generic_test<T: Clone>(value: T) {
            let foo = just(value);
            assert_eq!(size_of::<T>(), size_of_val(&foo));
        }
        generic_test("hi");
        generic_test("hi".to_string());
        generic_test(());
        generic_test(true);
        generic_test(42u8);
        generic_test(42i32);
        generic_test(42u128);
        generic_test('a');
        generic_test(Vec::<i32>::new());
        generic_test(std::collections::HashMap::<String, i32>::new());
        generic_test([0i32; 512]);
        generic_test(Box::new(4i16));
    }

    #[test]
    fn test_1() {
        let report = run_test_bool(
            |v: bool| !v,
            any(),
            &mut crate::rand::thread_rng(),
            TestConfig::default(),
        )
        .unwrap_report();
        assert_eq!(report.passing_runs, 1);
        let expected = vec![
            ShrinkStep::new(
                true,
                false,
                TestOutcome::Failed {
                    cause: FailureCause::NormalFailure,
                },
            ),
            ShrinkStep::new(false, false, TestOutcome::Passed),
        ];
        assert_eq!(report.shrink_steps.len(), expected.len());
        for (actual, expected) in report.shrink_steps.iter().zip(expected.iter()) {
            assert_eq!(actual.try_eq(expected), Some(true));
        }
        assert_eq!(report.simplest_failing_value(), &true);

        let report = run_test_bool(
            |v: bool| v,
            any(),
            &mut crate::rand::thread_rng(),
            TestConfig::default(),
        )
        .unwrap_report();
        assert_eq!(report.passing_runs, 0);
        let expected = vec![ShrinkStep::new(
            false,
            false,
            TestOutcome::Failed {
                cause: FailureCause::NormalFailure,
            },
        )];
        assert_eq!(report.shrink_steps.len(), expected.len());
        for (actual, expected) in report.shrink_steps.iter().zip(expected.iter()) {
            assert_eq!(actual.try_eq(expected), Some(true));
        }
        assert_eq!(report.simplest_failing_value(), &false);

        let report = run_test_bool(
            |_: bool| false,
            any(),
            &mut crate::rand::thread_rng(),
            TestConfig::default(),
        )
        .unwrap_report();
        assert_eq!(report.passing_runs, 0);
        let expected = vec![ShrinkStep::new(
            false,
            false,
            TestOutcome::Failed {
                cause: FailureCause::NormalFailure,
            },
        )];
        assert_eq!(report.shrink_steps.len(), expected.len());
        for (actual, expected) in report.shrink_steps.iter().zip(expected.iter()) {
            assert_eq!(actual.try_eq(expected), Some(true));
        }
        assert_eq!(report.simplest_failing_value(), &false);

        assert!(
            run_test_bool(
                |_: bool| true,
                any(),
                &mut crate::rand::thread_rng(),
                TestConfig::default(),
            )
            .passed()
        );
    }

    #[test]
    fn test_2() {
        let report = run_test_panic(
            |v: bool| assert!(!v),
            any(),
            &mut crate::rand::thread_rng(),
            TestConfig::default(),
        )
        .unwrap_report();

        match report.simplest_failing_cause() {
            FailureCause::ExpectedPanic { panic_data } => {
                assert_eq!(
                    panic_data.payload_as_string().unwrap(),
                    "assertion failed: !v"
                );
                assert_eq!(panic_data.location.as_ref().unwrap().file, file!());
            }
            _ => panic!(),
        }

        let report = run_test_panic(
            |v: bool| assert!(v, "My custom panic ({}) message [{}]", "at the disco", v),
            any(),
            &mut crate::rand::thread_rng(),
            TestConfig::default(),
        )
        .unwrap_report();

        match report.simplest_failing_cause() {
            FailureCause::ExpectedPanic { panic_data } => {
                assert_eq!(
                    panic_data.payload_as_string().unwrap(),
                    "My custom panic (at the disco) message [false]"
                );
                assert_eq!(panic_data.location.as_ref().unwrap().file, file!());
            }
            _ => panic!(),
        }
    }
}
