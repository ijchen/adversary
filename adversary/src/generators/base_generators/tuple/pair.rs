use crate::{report::Observation, shrinker::Shrinker, ValueGen};

pub struct Pair<'gens, Left: ValueGen, Right: ValueGen> {
    left_shrinker: Box<dyn Shrinker<Left::Seed> + 'gens>,
    right_shrinker: Box<dyn Shrinker<Right::Seed> + 'gens>,
    left_current_value: Left::Seed,
    right_current_value: Right::Seed,
}

impl<'gens, Left: ValueGen, Right: ValueGen> Pair<'gens, Left, Right> {
    pub fn new(
        left_shrinker: impl Shrinker<Left::Seed> + 'gens,
        right_shrinker: impl Shrinker<Right::Seed> + 'gens,
        left_current_value: Left::Seed,
        right_current_value: Right::Seed,
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

impl<Left: ValueGen, Right: ValueGen> Shrinker<(Left::Seed, Right::Seed)>
    for Pair<'_, Left, Right>
{
    fn current_attempt(&self) -> Option<(Left::Seed, Right::Seed)> {
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
