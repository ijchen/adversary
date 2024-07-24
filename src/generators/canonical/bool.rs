use crate::{
    input_generator::NextAttempt,
    report::{Importance, Observation},
    Canonical, InputGenerator,
};

struct CanonicalBoolGenerator;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
enum ObservedOutcomes {
    #[default]
    Nothing,
    Passed,
    Failed,
    Both,
}

impl ObservedOutcomes {
    pub fn observe_outcome(&mut self, passed: bool) {
        *self = match (*self, passed) {
            (Self::Nothing | Self::Passed, true) => Self::Passed,
            (Self::Nothing | Self::Failed, false) => Self::Failed,
            (Self::Failed, true) | (Self::Passed, false) | (Self::Both, _) => Self::Both,
        };
    }

    pub fn has_failed(self) -> bool {
        match self {
            Self::Nothing => false,
            Self::Passed => false,
            Self::Failed => true,
            Self::Both => true,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
struct BoolHistory {
    pub t: ObservedOutcomes,
    pub f: ObservedOutcomes,
}

impl InputGenerator for CanonicalBoolGenerator {
    type Input = bool;

    type History = BoolHistory;

    fn cardinality(&self) -> Option<usize> {
        Some(2)
    }

    fn exhaustive(&self) -> impl Iterator<Item = Self::Input> {
        [false, true].into_iter()
    }

    fn adversarial_count(&self) -> Option<usize> {
        Some(2)
    }

    fn adversarial(&self) -> impl Iterator<Item = Self::Input> {
        [false, true].into_iter()
    }

    fn sample(&self, rng: &mut (impl rand::Rng + ?Sized)) -> Self::Input {
        rng.gen()
    }

    fn new_history(&self) -> Self::History {
        Default::default()
    }

    fn update_history(&self, history: &mut Self::History, input: &Self::Input, test_passed: bool) {
        match input {
            true => history.t.observe_outcome(test_passed),
            false => history.f.observe_outcome(test_passed),
        }
    }

    fn generate_observations(&self, history: Self::History) -> Vec<Observation> {
        let mut observations = Vec::new();

        if history.f == ObservedOutcomes::Both {
            observations.push(Observation::new(
                "false was observed both passing and failing - possible non-deterministic behavior",
                Importance::MaybeRelevant,
            ));
        }

        if history.t == ObservedOutcomes::Both {
            observations.push(Observation::new(
                "true was observed both passing and failing - possible non-deterministic behavior",
                Importance::MaybeRelevant,
            ));
        }

        if history.t.has_failed() && history.f.has_failed() {
            observations.push(Observation::new(
                "both true and false were observed as failing - this value probably doesn't matter",
                Importance::MaybeRelevant,
            ));
        }

        observations
    }

    fn next_input(
        &self,
        _rng: &mut impl rand::Rng,
        history: &Self::History,
    ) -> NextAttempt<Self::Input> {
        // If we haven't tried false yet, try to shrink to it
        if history.f == ObservedOutcomes::Nothing {
            return NextAttempt::ShrinkAttempt(false);
        }

        // If we haven't tried true yet, try it for information
        if history.t == ObservedOutcomes::Nothing {
            return NextAttempt::InfoGathering(true);
        }

        NextAttempt::Done
    }
}

impl Canonical for bool {
    fn canonical() -> impl InputGenerator<Input = Self> {
        CanonicalBoolGenerator
    }
}
