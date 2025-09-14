mod elementwise;
use elementwise::Elementwise;

use crate::{
    ValueGen, generators::base_generators::tuple::Pair, report::Observation, shrinker::Shrinker,
};

pub struct TupleShrinker2<'gens, GenA: ValueGen, GenB: ValueGen> {
    generators: (&'gens GenA, &'gens GenB),
    current_values: (GenA::Seed, GenB::Seed),
    phase: Phase<'gens, GenA, GenB>,
}

enum Phase<'gens, GenA: ValueGen, GenB: ValueGen> {
    ElementwiseFirstPass(Elementwise<'gens, GenA, GenB>),
    TogetherFirstPass(Pair<'gens, GenA, GenB>),
    ElementwiseSecondPass(Elementwise<'gens, GenA, GenB>),
    TogetherSecondPass(Pair<'gens, GenA, GenB>),
    Done,
}

impl<'gens, GenA: ValueGen, GenB: ValueGen> TupleShrinker2<'gens, GenA, GenB> {
    pub fn new(
        generators: (&'gens GenA, &'gens GenB),
        current_values: (GenA::Seed, GenB::Seed),
    ) -> Self {
        let phase =
            Phase::ElementwiseFirstPass(Elementwise::new(generators, current_values.clone()));

        let mut this = Self {
            generators,
            current_values,
            phase,
        };

        this.progress_if_necessary();

        this
    }

    pub fn progress_if_necessary(&mut self) {
        // If ElementwiseFirstPass is done, progress to TogetherFirstPass
        if let Phase::ElementwiseFirstPass(phase) = &self.phase
            && phase.is_done()
        {
            self.phase = Phase::TogetherFirstPass(Pair::new(
                self.generators
                    .0
                    .new_shrinker(self.current_values.0.clone()),
                self.generators
                    .1
                    .new_shrinker(self.current_values.1.clone()),
                self.current_values.0.clone(),
                self.current_values.1.clone(),
            ));
        }

        // If TogetherFirstPass is done, progress to ElementwiseSecondPass
        if let Phase::TogetherFirstPass(phase) = &self.phase
            && phase.is_done()
        {
            self.phase = Phase::ElementwiseSecondPass(Elementwise::new(
                self.generators,
                self.current_values.clone(),
            ));
        }

        // If ElementwiseSecondPass is done, progress to TogetherSecondPass
        if let Phase::ElementwiseSecondPass(phase) = &self.phase
            && phase.is_done()
        {
            self.phase = Phase::TogetherSecondPass(Pair::new(
                self.generators
                    .0
                    .new_shrinker(self.current_values.0.clone()),
                self.generators
                    .1
                    .new_shrinker(self.current_values.1.clone()),
                self.current_values.0.clone(),
                self.current_values.1.clone(),
            ));
        }

        // If TogetherSecondPass is done, progress to Done
        if let Phase::TogetherSecondPass(phase) = &self.phase
            && phase.is_done()
        {
            self.phase = Phase::Done;
        }
    }
}

impl<GenA: ValueGen, GenB: ValueGen> Shrinker<(GenA::Seed, GenB::Seed)>
    for TupleShrinker2<'_, GenA, GenB>
{
    fn current_attempt(&self) -> Option<(GenA::Seed, GenB::Seed)> {
        match &self.phase {
            Phase::ElementwiseFirstPass(phase) => phase.current_attempt(),
            Phase::TogetherFirstPass(phase) => phase.current_attempt(),
            Phase::ElementwiseSecondPass(phase) => phase.current_attempt(),
            Phase::TogetherSecondPass(phase) => phase.current_attempt(),
            Phase::Done => None,
        }
    }

    fn update(&mut self, current_attempt_passed: bool) {
        if !current_attempt_passed {
            self.current_values = self.current_attempt().unwrap();
        }

        match &mut self.phase {
            Phase::ElementwiseFirstPass(phase) => {
                phase.update(self.generators, current_attempt_passed)
            }
            Phase::TogetherFirstPass(phase) => phase.update(current_attempt_passed),
            Phase::ElementwiseSecondPass(phase) => {
                phase.update(self.generators, current_attempt_passed)
            }
            Phase::TogetherSecondPass(phase) => phase.update(current_attempt_passed),
            Phase::Done => { /* Nothing to do here */ }
        }
    }

    fn into_observations(self) -> Vec<Observation> {
        // TODO(ichen): useful observations
        Vec::new()
    }
}
