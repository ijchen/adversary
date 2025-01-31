use crate::{
    report::{Importance, Observation},
    shrinker::Shrinker,
    ValueGen,
};

pub fn chance(chance_of_true: f64, shrink_to: bool) -> impl ValueGen<Value = bool> {
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
) -> impl ValueGen<Value = bool> {
    assert!(denominator != 0);
    assert!(numerator <= denominator);

    // TODO(ichen): do this in a way that doesn't just cast to a float (should
    // be able to just test `rng.gen_range(0..denominator) < numerator`)
    let chance_of_true = numerator as f64 / denominator as f64;

    chance(chance_of_true, shrink_to)
}

struct ChanceGen {
    // Should be in [0.0, 1.0]
    chance_of_true: f64,
    shrink_to: bool,
}

impl ValueGen for ChanceGen {
    type Value = bool;
    type Seed = Self::Value;
    // TODO: use ATPIT once stabilized
    type Shrinker = BoolShrinker;

    fn cardinality(&self) -> Option<usize> {
        Some(2)
    }

    fn exhaustive(&self) -> impl Iterator<Item = Self::Seed> {
        [false, true].into_iter()
    }

    fn adversarial_count(&self) -> Option<usize> {
        Some(2)
    }

    fn adversarial(&self) -> impl Iterator<Item = Self::Seed> {
        [false, true].into_iter()
    }

    fn sample(&self, rng: &mut (impl crate::rand::Rng + ?Sized)) -> Self::Seed {
        rng.gen_bool(self.chance_of_true)
    }

    fn new_shrinker(&self, failing_value_seed: Self::Seed) -> Self::Shrinker {
        let mut shrinker = BoolShrinker {
            shrink_to: self.shrink_to,
            t: Default::default(),
            f: Default::default(),
        };

        match failing_value_seed {
            true => &mut shrinker.t,
            false => &mut shrinker.f,
        }
        .observe_outcome(false);

        shrinker
    }

    fn create_value(&self, seed: Self::Seed) -> Self::Value {
        seed
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct BoolShrinker {
    shrink_to: bool,
    t: ObservedOutcomes,
    f: ObservedOutcomes,
}

impl Shrinker<ChanceGen> for BoolShrinker {
    fn current_attempt(&self) -> Option<<ChanceGen as ValueGen>::Seed> {
        // If we haven't tried our "shrink to" value yet, try it
        let shrink_to_observed = match self.shrink_to {
            true => self.t,
            false => self.f,
        };
        if shrink_to_observed == ObservedOutcomes::Nothing {
            return Some(self.shrink_to);
        }

        // TODO: do this again when informational shrink attempts are back
        // If we haven't tried the other (not "shrink to") yet, try it for
        // information
        // let other_observed = match self.shrink_to {
        //     true => self.f,
        //     false => self.t,
        // };
        // if other_observed == ObservedOutcomes::Nothing {
        //     return NextAttempt::InfoGathering(!self.shrink_to);
        // }

        // Nothing more to try, we're done here.
        None
    }

    fn update(&mut self, _gen: &ChanceGen, current_attempt_passed: bool) {
        let todo_current_attempt = self.current_attempt().unwrap();

        match todo_current_attempt {
            true => self.t.observe_outcome(current_attempt_passed),
            false => self.f.observe_outcome(current_attempt_passed),
        }
    }

    fn into_observations(self) -> Vec<Observation> {
        let mut observations = Vec::new();

        if self.f == ObservedOutcomes::Both {
            observations.push(Observation::new(
                "false was observed both passing and failing - possible non-deterministic behavior",
                Importance::MaybeRelevant,
            ));
        }

        if self.t == ObservedOutcomes::Both {
            observations.push(Observation::new(
                "true was observed both passing and failing - possible non-deterministic behavior",
                Importance::MaybeRelevant,
            ));
        }

        if self.t.has_failed() && self.f.has_failed() {
            observations.push(Observation::new(
                "both true and false were observed as failing - this value probably doesn't matter",
                Importance::MaybeRelevant,
            ));
        }

        observations
    }
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
