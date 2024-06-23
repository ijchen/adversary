mod adversarial;
mod exhaustive;
pub mod generators;
mod input_generator;
mod sample;
mod shrink;

pub use rand;

pub use adversarial::Adversarial;
pub use exhaustive::Exhaustive;
pub use input_generator::InputGenerator;
pub use sample::Sample;
pub use shrink::Shrink;

#[cfg(test)]
mod tests {
    use super::*;
    use generators::any;

    #[test]
    fn my_test() {
        const FAIL_THRESHOLD: u32 = 123454321;
        fn check(n: u32) -> bool {
            n < FAIL_THRESHOLD
        }

        const MAX_RUNS: usize = 1_000_000;

        let gen = any::<u32>();
        let rng = &mut rand::thread_rng();

        let mut failing_input = None;
        let inputs: Vec<u32> = if gen
            .cardinality()
            .is_some_and(|cardinality| cardinality <= MAX_RUNS)
        {
            gen.exhaustive().collect()
        } else if gen.adversarial_count() <= MAX_RUNS {
            gen.adversarial()
                .chain(std::iter::repeat_with(|| gen.sample(rng)).take(MAX_RUNS))
                .collect()
        } else {
            std::iter::repeat_with(|| gen.sample(rng))
                .take(MAX_RUNS)
                .collect()
        };
        for input in inputs {
            let passed = check(input);

            if !passed {
                failing_input = Some(input);
                break;
            }
        }
        let mut failing_input = failing_input.unwrap();

        eprintln!("initial failing input: {failing_input}");

        let mut history = gen.history_from_failure(failing_input);
        while let Some(next_input) = gen.next_input(rng, &history) {
            let passed = check(next_input);
            eprintln!(
                "next input: {next_input} ({})",
                if passed { "passed" } else { "failed" }
            );
            if !passed {
                failing_input = next_input
            }
            gen.update_history(&mut history, next_input, passed);
        }

        assert_eq!(failing_input, FAIL_THRESHOLD);
    }
}
