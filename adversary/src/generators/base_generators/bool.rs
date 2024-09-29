use crate::{
    report::{Importance, Observation},
    InputGenerator, NextAttempt,
};

struct ChanceGen {
    // Should be in [0.0, 1.0]
    chance_of_true: f64,
    shrink_to: bool,
}

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

impl InputGenerator for ChanceGen {
    type Input = bool;
    type InputSource = Self::Input;

    type History = BoolHistory;

    fn cardinality(&self) -> Option<usize> {
        Some(2)
    }

    fn exhaustive(&self) -> impl Iterator<Item = Self::InputSource> {
        [false, true].into_iter()
    }

    fn adversarial_count(&self) -> Option<usize> {
        Some(2)
    }

    fn adversarial(&self) -> impl Iterator<Item = Self::InputSource> {
        [false, true].into_iter()
    }

    fn sample(&self, rng: &mut (impl crate::rand::Rng + ?Sized)) -> Self::InputSource {
        rng.gen_bool(self.chance_of_true)
    }

    fn new_history(&self) -> Self::History {
        Default::default()
    }

    fn update_history(
        &self,
        history: &mut Self::History,
        shrinkable_input: Self::InputSource,
        test_passed: bool,
    ) {
        match shrinkable_input {
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
        _rng: &mut impl crate::rand::Rng,
        history: &Self::History,
    ) -> NextAttempt<Self::InputSource> {
        // If we haven't tried our "shrink to" value yet, try it
        let shrink_to_observed = match self.shrink_to {
            true => history.t,
            false => history.f,
        };
        if shrink_to_observed == ObservedOutcomes::Nothing {
            return NextAttempt::ShrinkAttempt(self.shrink_to);
        }

        // If we haven't tried the other (not "shrink to") yet, try it for information
        let other_observed = match self.shrink_to {
            true => history.f,
            false => history.t,
        };
        if other_observed == ObservedOutcomes::Nothing {
            return NextAttempt::InfoGathering(!self.shrink_to);
        }

        NextAttempt::Done
    }

    fn create_input(&self, input_source: Self::InputSource) -> Self::Input {
        input_source
    }
}

pub fn chance(chance_of_true: f64, shrink_to: bool) -> impl InputGenerator<Input = bool> {
    assert!((0.0..=1.0).contains(&chance_of_true));

    assert!(
        chance_of_true != 0.0 && chance_of_true != 1.0,
        "chance bool generators guaranteed to always return true or false are not yet implemented - use `just(true/false)` instead"
    );

    ChanceGen {
        chance_of_true,
        shrink_to,
    }
}

pub fn chance_ratio(
    numerator: u64,
    denominator: u64,
    shrink_to: bool,
) -> impl InputGenerator<Input = bool> {
    assert!(denominator != 0);
    assert!(numerator <= denominator);

    // TODO(ichen): do this in a way that doesn't just cast to a float (should
    // be able to just test `rng.gen_range(0..denominator) < numerator`)
    let chance_of_true = numerator as f64 / denominator as f64;

    chance(chance_of_true, shrink_to)
}
