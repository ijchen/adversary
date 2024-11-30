mod element_wise;

use element_wise::ElementWise3;

use crate::{report::Observation, shrinker::Shrinker, InputGenerator};

pub struct TupleShrinker3<'gens, GenA: InputGenerator, GenB: InputGenerator, GenC: InputGenerator> {
    generators: (&'gens GenA, &'gens GenB, &'gens GenC),
    current_values: (GenA::InputSource, GenB::InputSource, GenC::InputSource),
    phase: Phase3<'gens, GenA, GenB, GenC>,
}

enum Phase3<'gens, GenA: InputGenerator, GenB: InputGenerator, GenC: InputGenerator> {
    ElementWiseFirstPass(ElementWise3<'gens, GenA, GenB, GenC>),
    AllTogetherFirstPass,
    Pairwise,
    ElementWiseSecondPass(ElementWise3<'gens, GenA, GenB, GenC>),
    AllTogetherSecondPass,
    Done,
}

impl<'gens, GenA: InputGenerator, GenB: InputGenerator, GenC: InputGenerator>
    TupleShrinker3<'gens, GenA, GenB, GenC>
{
    pub fn new(
        generators: (&'gens GenA, &'gens GenB, &'gens GenC),
        current_values: (GenA::InputSource, GenB::InputSource, GenC::InputSource),
    ) -> Self {
        let phase =
            Phase3::ElementWiseFirstPass(ElementWise3::new(generators, current_values.clone()));

        let mut this = Self {
            generators,
            current_values,
            phase,
        };

        this.progress_if_necessary();

        this
    }

    pub fn progress_if_necessary(&mut self) {
        // If ElementWiseFirstPass is done, progress to AllTogetherFirstPass
        if let Phase3::ElementWiseFirstPass(phase) = &self.phase {
            if phase.is_done() {
                self.phase = Phase3::AllTogetherFirstPass;
            }
        }

        // If AllTogetherFirstPass is done, progress to Pairwise
        // TODO(ichen): until this phase is implemented, always progress
        if let Phase3::AllTogetherFirstPass = &self.phase {
            self.phase = Phase3::Pairwise;
        }

        // If Pairwise is done, progress to ElementWiseSecondPass
        // TODO(ichen): until this phase is implemented, always progress
        if let Phase3::Pairwise = &self.phase {
            self.phase = Phase3::ElementWiseSecondPass(ElementWise3::new(
                self.generators,
                self.current_values.clone(),
            ));
        }

        // If ElementWiseSecondPass is done, progress to AllTogetherSecondPass
        if let Phase3::ElementWiseSecondPass(phase) = &self.phase {
            if phase.is_done() {
                self.phase = Phase3::AllTogetherSecondPass;
            }
        }

        // If AllTogetherSecondPass is done, progress to Done
        // TODO(ichen): until this phase is implemented, always progress
        if let Phase3::AllTogetherSecondPass = &self.phase {
            self.phase = Phase3::Done;
        }
    }
}

impl<'gens, GenA: InputGenerator, GenB: InputGenerator, GenC: InputGenerator> Shrinker
    for TupleShrinker3<'gens, GenA, GenB, GenC>
{
    type InputSource = (GenA::InputSource, GenB::InputSource, GenC::InputSource);

    fn current_attempt(&self) -> Option<Self::InputSource> {
        match &self.phase {
            Phase3::ElementWiseFirstPass(phase) => phase.current_attempt(),
            Phase3::AllTogetherFirstPass => todo!(),
            Phase3::Pairwise => todo!(),
            Phase3::ElementWiseSecondPass(phase) => phase.current_attempt(),
            Phase3::AllTogetherSecondPass => todo!(),
            Phase3::Done => None,
        }
    }

    fn update(&mut self, current_attempt_passed: bool) {
        if !current_attempt_passed {
            self.current_values = self.current_attempt().unwrap();
        }

        match &mut self.phase {
            Phase3::ElementWiseFirstPass(phase) => {
                phase.update(self.generators, current_attempt_passed)
            }
            Phase3::AllTogetherFirstPass => todo!(),
            Phase3::Pairwise => todo!(),
            Phase3::ElementWiseSecondPass(phase) => {
                phase.update(self.generators, current_attempt_passed)
            }
            Phase3::AllTogetherSecondPass => todo!(),
            Phase3::Done => todo!(),
        }
    }

    fn into_observations(self) -> Vec<Observation> {
        // TODO: useful observations
        Vec::new()
    }
}
