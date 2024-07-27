use crate::{input_generator::NextAttempt, InputGenerator};

pub struct Map<G, F> {
    inner_generator: G,
    f: F,
}

impl<U, G: InputGenerator, F: Fn(G::Input) -> U> InputGenerator for Map<G, F> {
    type Input = U;
    type InputIdentifier = G::InputIdentifier;

    type History = G::History;

    fn cardinality(&self) -> Option<usize> {
        self.inner_generator.cardinality()
    }

    fn exhaustive(&self) -> impl Iterator<Item = (Self::Input, Self::InputIdentifier)> {
        self.inner_generator
            .exhaustive()
            .map(|(input, input_identifier)| ((self.f)(input), input_identifier))
    }

    fn adversarial_count(&self) -> Option<usize> {
        self.inner_generator.adversarial_count()
    }

    fn adversarial(&self) -> impl Iterator<Item = (Self::Input, Self::InputIdentifier)> {
        self.inner_generator
            .adversarial()
            .map(|(input, input_identifier)| ((self.f)(input), input_identifier))
    }

    fn sample(&self, rng: &mut (impl rand::Rng + ?Sized)) -> (Self::Input, Self::InputIdentifier) {
        let (input, input_identifier) = self.inner_generator.sample(rng);

        ((self.f)(input), input_identifier)
    }

    fn new_history(&self) -> Self::History {
        self.inner_generator.new_history()
    }

    fn next_input(
        &self,
        rng: &mut impl rand::Rng,
        history: &Self::History,
    ) -> NextAttempt<(Self::Input, Self::InputIdentifier)> {
        // TODO(ichen): consider impl'ing .map(...) on NextInput (that's what
        // I'm doing here, just manually)
        match self.inner_generator.next_input(rng, history) {
            NextAttempt::Done => NextAttempt::Done,
            NextAttempt::InfoGathering((input, input_identifier)) => {
                NextAttempt::InfoGathering(((self.f)(input), input_identifier))
            }
            NextAttempt::ShrinkAttempt((input, input_identifier)) => {
                NextAttempt::ShrinkAttempt(((self.f)(input), input_identifier))
            }
        }
    }

    fn update_history(
        &self,
        history: &mut Self::History,
        input_identifier: Self::InputIdentifier,
        test_passed: bool,
    ) {
        self.inner_generator
            .update_history(history, input_identifier, test_passed)
    }

    fn generate_observations(&self, history: Self::History) -> Vec<crate::report::Observation> {
        self.inner_generator.generate_observations(history)
    }
}

impl<U, G: InputGenerator, F: Fn(G::Input) -> U> Map<G, F> {
    pub fn new(inner_generator: G, map_function: F) -> Self {
        Self {
            inner_generator,
            f: map_function,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::run_test;

    #[test]
    fn test_map_does_the_map_thing() {
        let report = run_test(
            |n| n.len() == 1,
            Map::new(
                [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14].as_slice(),
                |t| t.to_string(),
            ),
            &mut crate::rand::thread_rng(),
        )
        .unwrap_err();
        assert_eq!(report.passing_runs, 10);
        assert_eq!(report.shrink_steps, vec![]);
        assert_eq!(report.simplest_failing_input, "10");
    }
}
