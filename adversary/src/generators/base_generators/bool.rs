use crate::{
    Chance, RangeAwareValueGen, ValueGen,
    report::{Importance, Observation},
    shrinker::Shrinker,
};

pub fn chance(chance_of_true: Chance, shrink_to: bool) -> impl ValueGen<Value = bool> {
    ChanceGen {
        chance_of_true,
        shrink_to,
    }
}

struct ChanceGen {
    chance_of_true: Chance,
    shrink_to: bool,
}

impl ValueGen for ChanceGen {
    type Value = bool;
    type Seed = Self::Value;
    // TODO: use ATPIT once stabilized
    type Shrinker<'a>
        = BoolShrinker
    where
        Self: 'a;

    fn cardinality(&self) -> Option<usize> {
        match self.chance_of_true {
            Chance::IMPOSSIBLE | Chance::GUARANTEED => Some(1),
            _ => Some(2),
        }
    }

    fn exhaustive(&self) -> impl Iterator<Item = Self::Seed> {
        match self.chance_of_true {
            Chance::IMPOSSIBLE => [false].as_slice(),
            Chance::GUARANTEED => [true].as_slice(),
            _ => [false, true].as_slice(),
        }
        .into_iter()
        .copied()
    }

    fn adversarial_count(&self) -> Option<usize> {
        self.cardinality()
    }

    fn adversarial(&self) -> impl Iterator<Item = Self::Seed> {
        self.exhaustive()
    }

    fn sample(&self, rng: &mut (impl crate::rand::Rng + ?Sized)) -> Self::Seed {
        self.chance_of_true.gen_bool(rng)
    }

    fn new_shrinker(&self, failing_value_seed: Self::Seed) -> Self::Shrinker<'_> {
        // If we're guaranteed to always return true or false, there's no point shrinking (and in
        // fact, the current implementation of BoolShrinker assumes both values are possible)
        if matches!(self.chance_of_true, Chance::IMPOSSIBLE | Chance::GUARANTEED) {
            return BoolShrinker::DoNotShrink;
        }

        let mut t = ObservedOutcomes::Nothing;
        let mut f = ObservedOutcomes::Nothing;
        match failing_value_seed {
            true => &mut t,
            false => &mut f,
        }
        .observe_outcome(false);

        BoolShrinker::Shrink {
            shrink_to: self.shrink_to,
            t,
            f,
        }
    }

    fn create_value(&self, seed: Self::Seed) -> Self::Value {
        seed
    }
}

impl RangeAwareValueGen for ChanceGen {
    fn value_in_range(&self, value: &Self::Value) -> bool {
        match value {
            true => self.chance_of_true.is_possible(),
            false => !self.chance_of_true.is_guaranteed(),
        }
    }
}

#[derive(Debug)]
enum BoolShrinker {
    DoNotShrink,
    Shrink {
        shrink_to: bool,
        t: ObservedOutcomes,
        f: ObservedOutcomes,
    },
}

impl Shrinker<bool> for BoolShrinker {
    fn current_attempt(&self) -> Option<bool> {
        let Self::Shrink { shrink_to, t, f } = self else {
            return None;
        };

        // If we haven't tried our "shrink to" value yet, try it
        let shrink_to_observed = match shrink_to {
            true => t,
            false => f,
        };
        if shrink_to_observed == &ObservedOutcomes::Nothing {
            return Some(*shrink_to);
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

    fn update(&mut self, current_attempt_passed: bool) {
        let Some(todo_current_attempt) = self.current_attempt() else {
            panic!("`BoolShrinker::update` called while done shrinking");
        };

        let Self::Shrink { t, f, .. } = self else {
            unreachable!()
        };

        match todo_current_attempt {
            true => t.observe_outcome(current_attempt_passed),
            false => f.observe_outcome(current_attempt_passed),
        }
    }

    fn into_observations(self) -> Vec<Observation> {
        let Self::Shrink { t, f, .. } = self else {
            return Vec::new();
        };

        let mut observations = Vec::new();

        if f == ObservedOutcomes::Both {
            observations.push(Observation::new(
                "false was observed both passing and failing - possible non-deterministic behavior",
                Importance::MaybeRelevant,
            ));
        }

        if t == ObservedOutcomes::Both {
            observations.push(Observation::new(
                "true was observed both passing and failing - possible non-deterministic behavior",
                Importance::MaybeRelevant,
            ));
        }

        if t.has_failed() && f.has_failed() {
            observations.push(Observation::new(
                "both true and false were observed as failing - this value probably doesn't matter",
                Importance::MaybeRelevant,
            ));
        }

        observations
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ObservedOutcomes {
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
