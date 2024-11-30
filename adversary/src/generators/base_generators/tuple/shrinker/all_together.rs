use crate::{shrinker::Shrinker, InputGenerator};

#[expect(
    clippy::type_complexity,
    reason = "weird code will eventually be the output of a macro, and this type is private anyway"
)]
pub struct AllTogether3<'gens, GenA: InputGenerator, GenB: InputGenerator, GenC: InputGenerator> {
    current_values: (GenA::InputSource, GenB::InputSource, GenC::InputSource),
    shrinkers: (
        Box<dyn Shrinker<InputSource = GenA::InputSource> + 'gens>,
        Box<dyn Shrinker<InputSource = GenB::InputSource> + 'gens>,
        Box<dyn Shrinker<InputSource = GenC::InputSource> + 'gens>,
    ),
}

impl<'gens, GenA: InputGenerator, GenB: InputGenerator, GenC: InputGenerator>
    AllTogether3<'gens, GenA, GenB, GenC>
{
    pub fn new(
        generators: (&'gens GenA, &'gens GenB, &'gens GenC),
        current_values: (GenA::InputSource, GenB::InputSource, GenC::InputSource),
    ) -> Self {
        let shrinkers = (
            Box::new(generators.0.new_shrinker(current_values.0.clone())) as _,
            Box::new(generators.1.new_shrinker(current_values.1.clone())) as _,
            Box::new(generators.2.new_shrinker(current_values.2.clone())) as _,
        );

        Self {
            current_values,
            shrinkers,
        }
    }

    pub fn is_done(&self) -> bool {
        self.shrinkers.0.current_attempt().is_none()
            && self.shrinkers.1.current_attempt().is_none()
            && self.shrinkers.2.current_attempt().is_none()
    }

    pub fn current_attempt(
        &self,
    ) -> Option<(GenA::InputSource, GenB::InputSource, GenC::InputSource)> {
        match (
            self.shrinkers.0.current_attempt(),
            self.shrinkers.1.current_attempt(),
            self.shrinkers.2.current_attempt(),
        ) {
            // If all shrinkers are done, so are we
            (None, None, None) => None,

            // As long as any shrinker can make progress, keep trying
            attempts => Some((
                attempts.0.unwrap_or_else(|| self.current_values.0.clone()),
                attempts.1.unwrap_or_else(|| self.current_values.1.clone()),
                attempts.2.unwrap_or_else(|| self.current_values.2.clone()),
            )),
        }
    }

    pub fn update(&mut self, current_attempt_passed: bool) {
        match (
            self.shrinkers.0.current_attempt(),
            self.shrinkers.1.current_attempt(),
            self.shrinkers.2.current_attempt(),
        ) {
            (None, None, None) => {
                panic!("`AllTogether3::update` called while all shrinkers were done")
            }

            // TODO(ichen): I think there's some kinda weird implications here
            // where we're telling shrinkers that an attempt passed or failed,
            // but that passing or failure may have nothing to do with their
            // attempt. I could see this leading to weird or even problematic
            // behavior... should investigate.
            attempts => {
                if attempts.0.is_some() {
                    self.shrinkers.0.update(current_attempt_passed);
                }
                if attempts.1.is_some() {
                    self.shrinkers.1.update(current_attempt_passed);
                }
                if attempts.2.is_some() {
                    self.shrinkers.2.update(current_attempt_passed);
                }
            }
        }
    }
}
