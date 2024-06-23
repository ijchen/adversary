use crate::{Adversarial, Exhaustive, InputGenerator, Sample, Shrink};

use super::Canonical;

impl Exhaustive<()> for Canonical {
    fn cardinality(&self) -> Option<usize> {
        Some(1)
    }

    fn exhaustive(&self) -> impl Iterator<Item = ()> {
        std::iter::once(())
    }
}

impl Adversarial<()> for Canonical {
    fn adversarial_count(&self) -> usize {
        1
    }

    fn adversarial(&self) -> impl Iterator<Item = ()> {
        std::iter::once(())
    }
}

impl Sample<()> for Canonical {
    fn sample(&self, _rng: &mut impl rand::Rng) -> () {
        ()
    }
}

impl Shrink<()> for Canonical {
    type History = ();

    fn history_from_failure(&self, _failing_input: ()) -> Self::History {
        todo!()
    }

    fn update_history(&self, _history: &mut Self::History, _input: (), _test_passed: bool) {
        // Nothing to do here
    }

    fn next_input(&self, _rng: &mut impl rand::Rng, _history: &Self::History) -> Option<()> {
        // We never need to shrink unit
        None
    }
}

impl InputGenerator<()> for Canonical {}
