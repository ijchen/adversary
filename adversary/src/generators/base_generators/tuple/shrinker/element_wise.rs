use crate::{shrinker::Shrinker, InputGenerator};

pub struct ElementWise3<'gens, GenA: InputGenerator, GenB: InputGenerator, GenC: InputGenerator> {
    current_values: (GenA::InputSource, GenB::InputSource, GenC::InputSource),
    step: Step3<'gens, GenA, GenB, GenC>,
}

enum Step3<'gens, GenA: InputGenerator, GenB: InputGenerator, GenC: InputGenerator> {
    ShrinkingA(Box<dyn Shrinker<InputSource = GenA::InputSource> + 'gens>),
    ShrinkingB(Box<dyn Shrinker<InputSource = GenB::InputSource> + 'gens>),
    ShrinkingC(Box<dyn Shrinker<InputSource = GenC::InputSource> + 'gens>),
    Done,
}

impl<'gens, GenA: InputGenerator, GenB: InputGenerator, GenC: InputGenerator>
    ElementWise3<'gens, GenA, GenB, GenC>
{
    pub fn new(
        generators: (&'gens GenA, &'gens GenB, &'gens GenC),
        current_values: (GenA::InputSource, GenB::InputSource, GenC::InputSource),
    ) -> Self {
        let step = Step3::ShrinkingA(Box::new(
            generators.0.new_shrinker(current_values.0.clone()),
        ));

        let mut this = Self {
            current_values,
            step,
        };

        this.progress_if_necessary(generators);

        this
    }

    pub fn progress_if_necessary(&mut self, generators: (&'gens GenA, &'gens GenB, &'gens GenC)) {
        // If the A shrinker is done, progress to B
        if let Step3::ShrinkingA(shrinker) = &mut self.step {
            if shrinker.current_attempt().is_none() {
                self.step = Step3::ShrinkingB(Box::new(
                    generators.1.new_shrinker(self.current_values.1.clone()),
                ));
            }
        }

        // If the B shrinker is done, progress to C
        if let Step3::ShrinkingB(shrinker) = &mut self.step {
            if shrinker.current_attempt().is_none() {
                self.step = Step3::ShrinkingC(Box::new(
                    generators.2.new_shrinker(self.current_values.2.clone()),
                ));
            }
        }

        // If the C shrinker is done, progress to Done
        if let Step3::ShrinkingC(shrinker) = &mut self.step {
            if shrinker.current_attempt().is_none() {
                self.step = Step3::Done;
            }
        }
    }

    pub fn is_done(&self) -> bool {
        matches!(self.step, Step3::Done)
    }

    pub fn current_attempt(
        &self,
    ) -> Option<(GenA::InputSource, GenB::InputSource, GenC::InputSource)> {
        match &self.step {
            Step3::ShrinkingA(shrinker) => Some((
                shrinker.current_attempt().unwrap(),
                self.current_values.1.clone(),
                self.current_values.2.clone(),
            )),
            Step3::ShrinkingB(shrinker) => Some((
                self.current_values.0.clone(),
                shrinker.current_attempt().unwrap(),
                self.current_values.2.clone(),
            )),
            Step3::ShrinkingC(shrinker) => Some((
                self.current_values.0.clone(),
                self.current_values.1.clone(),
                shrinker.current_attempt().unwrap(),
            )),
            Step3::Done => None,
        }
    }

    pub fn update(
        &mut self,
        generators: (&'gens GenA, &'gens GenB, &'gens GenC),
        current_attempt_passed: bool,
    ) {
        match &mut self.step {
            Step3::ShrinkingA(shrinker) => shrinker.update(current_attempt_passed),
            Step3::ShrinkingB(shrinker) => shrinker.update(current_attempt_passed),
            Step3::ShrinkingC(shrinker) => shrinker.update(current_attempt_passed),
            Step3::Done => panic!("`ElementWise3::update` called while in `Step::Done`"),
        }

        self.progress_if_necessary(generators);
    }
}
