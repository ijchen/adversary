use crate::{shrinker::Shrinker, InputGenerator};

pub struct Elementwise<'gens, GenA: InputGenerator, GenB: InputGenerator> {
    current_values: (GenA::InputSource, GenB::InputSource),
    step: Step<'gens, GenA, GenB>,
}

enum Step<'gens, GenA: InputGenerator, GenB: InputGenerator> {
    ShrinkingA(Box<dyn Shrinker<InputSource = GenA::InputSource> + 'gens>),
    ShrinkingB(Box<dyn Shrinker<InputSource = GenB::InputSource> + 'gens>),
    Done,
}

impl<'gens, GenA: InputGenerator, GenB: InputGenerator> Elementwise<'gens, GenA, GenB> {
    pub fn new(
        generators: (&'gens GenA, &'gens GenB),
        current_values: (GenA::InputSource, GenB::InputSource),
    ) -> Self {
        let step = Step::ShrinkingA(Box::new(
            generators.0.new_shrinker(current_values.0.clone()),
        ));

        let mut this = Self {
            current_values,
            step,
        };

        this.progress_if_necessary(generators);

        this
    }

    pub fn progress_if_necessary(&mut self, generators: (&'gens GenA, &'gens GenB)) {
        // If the first shrinker is done, progress to the second
        if let Step::ShrinkingA(shrinker) = &mut self.step {
            if shrinker.current_attempt().is_none() {
                self.step = Step::ShrinkingB(Box::new(
                    generators.1.new_shrinker(self.current_values.1.clone()),
                ));
            }
        }

        // If the second shrinker is done, progress to Done
        if let Step::ShrinkingB(shrinker) = &mut self.step {
            if shrinker.current_attempt().is_none() {
                self.step = Step::Done;
            }
        }
    }

    pub fn is_done(&self) -> bool {
        matches!(self.step, Step::Done)
    }

    pub fn current_attempt(&self) -> Option<(GenA::InputSource, GenB::InputSource)> {
        match &self.step {
            Step::ShrinkingA(shrinker) => Some((
                shrinker.current_attempt().unwrap(),
                self.current_values.1.clone(),
            )),
            Step::ShrinkingB(shrinker) => Some((
                self.current_values.0.clone(),
                shrinker.current_attempt().unwrap(),
            )),
            Step::Done => None,
        }
    }

    pub fn update(&mut self, generators: (&'gens GenA, &'gens GenB), current_attempt_passed: bool) {
        match &mut self.step {
            Step::ShrinkingA(shrinker) => shrinker.update(current_attempt_passed),
            Step::ShrinkingB(shrinker) => shrinker.update(current_attempt_passed),
            Step::Done => panic!(concat!(
                "`Elementwise::update` called while in `Step::Done`"
            )),
        }

        self.progress_if_necessary(generators);
    }
}
