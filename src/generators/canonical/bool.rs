use crate::{Adversarial, Exhaustive, InputGenerator, Sample, Shrink};

use super::Canonical;

impl Exhaustive<bool> for Canonical {
    fn cardinality(&self) -> Option<usize> {
        Some(2)
    }

    fn exhaustive(&self) -> impl Iterator<Item = bool> {
        [true, false].into_iter()
    }
}

impl Adversarial<bool> for Canonical {
    fn adversarial_count(&self) -> usize {
        2
    }

    fn adversarial(&self) -> impl Iterator<Item = bool> {
        [true, false].into_iter()
    }
}

impl Sample<bool> for Canonical {
    fn sample(&self, rng: &mut impl rand::Rng) -> bool {
        rng.gen()
    }
}

#[derive(Debug, Clone, Copy)]
enum BoolHistoryInner {
    TruePassFalseFail,
    TrueUnknFalseFail,
    TrueFailFalsePass,
    TrueFailFalseUnkn,
    TrueFailFalseFail,
}

#[derive(Debug)]
pub struct BoolHistory {
    inner: BoolHistoryInner,
}

// TODO: do a much more thoughtful and improved implementation - this is just a
// basic impl to get started with.
impl Shrink<bool> for Canonical {
    type History = BoolHistory;

    fn history_from_failure(&self, failing_input: bool) -> Self::History {
        match failing_input {
            true => BoolHistory {
                inner: BoolHistoryInner::TrueFailFalseUnkn,
            },
            false => BoolHistory {
                inner: BoolHistoryInner::TrueUnknFalseFail,
            },
        }
    }

    fn update_history(&self, history: &mut Self::History, input: bool, test_passed: bool) {
        use BoolHistoryInner as B;

        history.inner = match (history.inner, input, test_passed) {
            // No new information
            (B::TruePassFalseFail | B::TrueFailFalsePass | B::TrueFailFalseFail, _, _) => {
                history.inner
            }
            (B::TrueUnknFalseFail, false, _) | (B::TrueFailFalseUnkn, true, _) => history.inner,

            // Some new information
            (B::TrueUnknFalseFail, true, true) => B::TruePassFalseFail,
            (B::TrueUnknFalseFail, true, false) => B::TrueFailFalseFail,
            (B::TrueFailFalseUnkn, false, true) => B::TrueFailFalsePass,
            (B::TrueFailFalseUnkn, false, false) => B::TrueFailFalseFail,
        }
    }

    fn next_input(&self, _rng: &mut impl rand::Rng, history: &Self::History) -> Option<bool> {
        use BoolHistoryInner as B;

        match history.inner {
            B::TrueFailFalseUnkn => Some(false),
            B::TruePassFalseFail
            | B::TrueUnknFalseFail
            | B::TrueFailFalsePass
            | B::TrueFailFalseFail => None,
        }
    }
}

impl InputGenerator<bool> for Canonical {}
