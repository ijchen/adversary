use crate::{shrinker::Shrinker, InputGenerator};

pub struct Pairwise3<'gens, GenA: InputGenerator, GenB: InputGenerator, GenC: InputGenerator> {
    current_values: (GenA::InputSource, GenB::InputSource, GenC::InputSource),
    step: Step3<'gens, GenA, GenB, GenC>,
}

// TODO(ichen): is there a nicer way to do this than an enum for every possible
// combination of two generators?
enum Step3<'gens, GenA: InputGenerator, GenB: InputGenerator, GenC: InputGenerator> {
    ShrinkingAB(Pair3<'gens, GenA, GenB>),
    ShrinkingAC(Pair3<'gens, GenA, GenC>),
    ShrinkingBC(Pair3<'gens, GenB, GenC>),
    Done,
}

impl<'gens, GenA: InputGenerator, GenB: InputGenerator, GenC: InputGenerator>
    Pairwise3<'gens, GenA, GenB, GenC>
{
    pub fn new(
        generators: (&'gens GenA, &'gens GenB, &'gens GenC),
        current_values: (GenA::InputSource, GenB::InputSource, GenC::InputSource),
    ) -> Self {
        let step = Step3::ShrinkingAB(Pair3::new(
            generators.0.new_shrinker(current_values.0.clone()),
            generators.1.new_shrinker(current_values.1.clone()),
            current_values.0.clone(),
            current_values.1.clone(),
        ));

        let mut this = Self {
            current_values,
            step,
        };

        this.progress_if_necessary(generators);

        this
    }

    pub fn progress_if_necessary(&mut self, generators: (&'gens GenA, &'gens GenB, &'gens GenC)) {
        // If the AB pair is done, progress to AC
        if let Step3::ShrinkingAB(pair) = &mut self.step {
            if pair.is_done() {
                self.step = Step3::ShrinkingAC(Pair3::new(
                    generators.0.new_shrinker(self.current_values.0.clone()),
                    generators.2.new_shrinker(self.current_values.2.clone()),
                    self.current_values.0.clone(),
                    self.current_values.2.clone(),
                ));
            }
        }

        // If the AC pair is done, progress to BC
        if let Step3::ShrinkingAC(pair) = &mut self.step {
            if pair.is_done() {
                self.step = Step3::ShrinkingBC(Pair3::new(
                    generators.1.new_shrinker(self.current_values.1.clone()),
                    generators.2.new_shrinker(self.current_values.2.clone()),
                    self.current_values.1.clone(),
                    self.current_values.2.clone(),
                ));
            }
        }

        // If the BC pair is done, progress to Done
        if let Step3::ShrinkingBC(pair) = &mut self.step {
            if pair.is_done() {
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
            Step3::ShrinkingAB(pair) => {
                let (a, b) = pair.current_attempt().unwrap();
                Some((a, b, self.current_values.2.clone()))
            }
            Step3::ShrinkingAC(pair) => {
                let (a, c) = pair.current_attempt().unwrap();
                Some((a, self.current_values.1.clone(), c))
            }
            Step3::ShrinkingBC(pair) => {
                let (b, c) = pair.current_attempt().unwrap();
                Some((self.current_values.0.clone(), b, c))
            }
            Step3::Done => None,
        }
    }

    pub fn update(
        &mut self,
        generators: (&'gens GenA, &'gens GenB, &'gens GenC),
        current_attempt_passed: bool,
    ) {
        match &mut self.step {
            Step3::ShrinkingAB(pair) => pair.update(current_attempt_passed),
            Step3::ShrinkingAC(pair) => pair.update(current_attempt_passed),
            Step3::ShrinkingBC(pair) => pair.update(current_attempt_passed),
            Step3::Done => panic!("`ElementWise3::update` called while in `Step::Done`"),
        }

        self.progress_if_necessary(generators);
    }
}

struct Pair3<'gens, Left: InputGenerator, Right: InputGenerator> {
    left_shrinker: Box<dyn Shrinker<InputSource = Left::InputSource> + 'gens>,
    right_shrinker: Box<dyn Shrinker<InputSource = Right::InputSource> + 'gens>,
    left_current_value: Left::InputSource,
    right_current_value: Right::InputSource,
}

impl<'gens, Left: InputGenerator, Right: InputGenerator> Pair3<'gens, Left, Right> {
    pub fn new(
        left_shrinker: impl Shrinker<InputSource = Left::InputSource> + 'gens,
        right_shrinker: impl Shrinker<InputSource = Right::InputSource> + 'gens,
        left_current_value: Left::InputSource,
        right_current_value: Right::InputSource,
    ) -> Self {
        Self {
            left_shrinker: Box::new(left_shrinker),
            right_shrinker: Box::new(right_shrinker),
            left_current_value,
            right_current_value,
        }
    }

    pub fn is_done(&self) -> bool {
        self.left_shrinker.current_attempt().is_none()
            && self.right_shrinker.current_attempt().is_none()
    }
}

impl<'gens, Left: InputGenerator, Right: InputGenerator> Shrinker for Pair3<'gens, Left, Right> {
    type InputSource = (Left::InputSource, Right::InputSource);

    fn current_attempt(&self) -> Option<Self::InputSource> {
        match (
            self.left_shrinker.current_attempt(),
            self.right_shrinker.current_attempt(),
        ) {
            (None, None) => None,
            (left_attempt, right_attempt) => Some((
                left_attempt.unwrap_or_else(|| self.left_current_value.clone()),
                right_attempt.unwrap_or_else(|| self.right_current_value.clone()),
            )),
        }
    }

    fn update(&mut self, current_attempt_passed: bool) {
        match (
            self.left_shrinker.current_attempt(),
            self.right_shrinker.current_attempt(),
        ) {
            (None, None) => {
                panic!("`Pair3::update` called while both shrinkers were done")
            }

            // TODO(ichen): I think there's some kinda weird implications here
            // where we're telling either shrinker that an attempt passed or
            // failed, but that passing or failure may have nothing to do with
            // its attempt. I could see this leading to weird or even
            // problematic behavior... should investigate.
            attempts => {
                if attempts.0.is_some() {
                    self.left_shrinker.update(current_attempt_passed);
                }
                if attempts.1.is_some() {
                    self.right_shrinker.update(current_attempt_passed);
                }
            }
        }
    }

    fn into_observations(self) -> Vec<crate::report::Observation> {
        Vec::new()
    }
}
