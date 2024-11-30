use crate::InputGenerator;

pub fn map<G: InputGenerator, U>(
    inner_generator: G,
    map_function: impl Fn(G::Input) -> U,
) -> impl InputGenerator<Input = U, InputSource = G::InputSource> {
    Map {
        inner_generator,
        f: map_function,
    }
}

struct Map<G, F> {
    inner_generator: G,
    f: F,
}

impl<U, G: InputGenerator, F: Fn(G::Input) -> U> InputGenerator for Map<G, F> {
    type Input = U;
    type InputSource = G::InputSource;

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

    fn new_shrinker(
        &self,
        failing_input: Self::InputSource,
    ) -> impl crate::shrinker::Shrinker<InputSource = Self::InputSource> {
        self.inner_generator.new_shrinker(failing_input)
    }

    fn create_input(&self, input_source: Self::InputSource) -> Self::Input {
        (self.f)(self.inner_generator.create_input(input_source))
    }
}

#[cfg(test)]
mod tests {
    use crate::{run_test, InputGeneratorExt, IntoInputGenerator, ShrinkStep};

    #[test]
    fn test_map_does_the_map_thing() {
        let report = run_test(
            |n| n.len() == 1,
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]
                .into_input_generator()
                .adv_map(|t| t.to_string()),
            &mut crate::rand::thread_rng(),
        )
        .unwrap_err();
        assert_eq!(report.passing_runs, 10);
        assert_eq!(
            report.shrink_steps,
            vec![
                ShrinkStep::new("0".to_string(), false, true),
                ShrinkStep::new("1".to_string(), false, true),
                ShrinkStep::new("2".to_string(), false, true),
                ShrinkStep::new("3".to_string(), false, true),
                ShrinkStep::new("4".to_string(), false, true),
                ShrinkStep::new("5".to_string(), false, true),
                ShrinkStep::new("6".to_string(), false, true),
                ShrinkStep::new("7".to_string(), false, true),
                ShrinkStep::new("8".to_string(), false, true),
                ShrinkStep::new("9".to_string(), false, true),
            ]
        );
        assert_eq!(report.simplest_failing_input, "10");
    }
}
