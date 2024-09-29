use crate::{input_generator::NextAttempt, InputGenerator};

pub struct Map<G, F> {
    inner_generator: G,
    f: F,
}

impl<U, G: InputGenerator, F: Fn(G::Input) -> U> InputGenerator for Map<G, F> {
    type Input = U;
    type InputSource = G::InputSource;

    type History = G::History;

    fn cardinality(&self) -> Option<usize> {
        self.inner_generator.cardinality()
    }

    fn exhaustive(&self) -> impl Iterator<Item = Self::InputSource> {
        self.inner_generator.exhaustive()
    }

    fn adversarial_count(&self) -> Option<usize> {
        self.inner_generator.adversarial_count()
    }

    fn adversarial(&self) -> impl Iterator<Item = Self::InputSource> {
        self.inner_generator.adversarial()
    }

    fn sample(&self, rng: &mut (impl rand::Rng + ?Sized)) -> Self::InputSource {
        self.inner_generator.sample(rng)
    }

    fn new_history(&self) -> Self::History {
        self.inner_generator.new_history()
    }

    fn next_input(
        &self,
        rng: &mut impl rand::Rng,
        history: &Self::History,
    ) -> NextAttempt<Self::InputSource> {
        self.inner_generator.next_input(rng, history)
    }

    fn update_history(
        &self,
        history: &mut Self::History,
        shrinkable_input: Self::InputSource,
        test_passed: bool,
    ) {
        self.inner_generator
            .update_history(history, shrinkable_input, test_passed)
    }

    fn generate_observations(&self, history: Self::History) -> Vec<crate::report::Observation> {
        self.inner_generator.generate_observations(history)
    }

    fn create_input(&self, input_source: Self::InputSource) -> Self::Input {
        (self.f)(self.inner_generator.create_input(input_source))
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
