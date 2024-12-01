use crate::{report::Observation, shrinker::Shrinker, InputGenerator};

pub struct Pair<'gens, Left: InputGenerator, Right: InputGenerator> {
    left_shrinker: Box<dyn Shrinker<InputSource = Left::InputSource> + 'gens>,
    right_shrinker: Box<dyn Shrinker<InputSource = Right::InputSource> + 'gens>,
    left_current_value: Left::InputSource,
    right_current_value: Right::InputSource,
}

impl<'gens, Left: InputGenerator, Right: InputGenerator> Pair<'gens, Left, Right> {
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

impl<'gens, Left: InputGenerator, Right: InputGenerator> Shrinker for Pair<'gens, Left, Right> {
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

    fn into_observations(self) -> Vec<Observation> {
        Vec::new()
    }
}
