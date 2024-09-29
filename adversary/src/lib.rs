pub mod generators;
mod input_generator;
mod input_generator_ext;
mod into_input_generator;
pub mod prelude;
mod report;
mod test_runner;

#[cfg(feature = "macros")]
pub use adversary_macros::adv_test;

// TODO(ichen): I don't really want to re-export this whole crate - we only need
// rand::Rng for InputGenerator. Instead, have our own Rng trait.
pub use rand;

pub use generators::{any, bool, just, just_with, Canonical};
pub use input_generator::{InputGenerator, NextAttempt};
pub use input_generator_ext::InputGeneratorExt;
pub use into_input_generator::IntoInputGenerator;
// TODO: don't publicly re-export Plaintext
pub use report::{Plaintext, Report, ShrinkStep};
pub use test_runner::{run_test, run_test_panics};

#[cfg(test)]
mod tests {
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
        let report = run_test(|v: bool| !v, any(), &mut crate::rand::thread_rng()).unwrap_err();
        assert_eq!(report.passing_runs, 1);
        assert_eq!(
            report.shrink_steps,
            vec![ShrinkStep::new(false, false, true)]
        );
        assert_eq!(report.simplest_failing_input, true);

        let report = run_test(|v: bool| v, any(), &mut crate::rand::thread_rng()).unwrap_err();
        assert_eq!(report.passing_runs, 0);
        assert_eq!(report.shrink_steps, vec![ShrinkStep::new(true, true, true)]);
        assert_eq!(report.simplest_failing_input, false);

        let report = run_test(|_: bool| false, any(), &mut crate::rand::thread_rng()).unwrap_err();
        assert_eq!(report.passing_runs, 0);
        assert_eq!(
            report.shrink_steps,
            vec![ShrinkStep::new(true, true, false)]
        );
        assert_eq!(report.simplest_failing_input, false);

        run_test(|_: bool| true, any(), &mut crate::rand::thread_rng()).unwrap();
    }

    #[test]
    fn test_2() {
        let report = run_test_panics(|v: bool| assert!(!v), any(), &mut crate::rand::thread_rng())
            .unwrap_err();
        assert_eq!(
            report.panic_message,
            Some("assertion failed: !v".to_string())
        );

        let report = run_test_panics(
            |v: bool| assert!(v, "My custom panic ({}) message [{}]", "at the disco", v),
            any(),
            &mut crate::rand::thread_rng(),
        )
        .unwrap_err();
        assert_eq!(
            report.panic_message,
            Some("My custom panic (at the disco) message [false]".to_string())
        );
    }
}
