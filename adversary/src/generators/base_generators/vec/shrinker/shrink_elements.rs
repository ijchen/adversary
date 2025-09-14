use crate::{
    ValueGen,
    shrinker::Shrinker as _,
    vec::shrinker::{VecShrinker, done::Done},
};

#[derive(Debug)]
pub struct ShrinkElements<'value_gen, G: ValueGen> {
    value_gen: &'value_gen G,
    simplest_known_failing: Box<[G::Seed]>,
    index: usize, // index == simplest_known_failing.len() indicates we're done shrinking
    // Invariant: if self isn't done shrinking (our .current_attempt() returns `Some`), neither is
    // elem_shrinker
    elem_shrinker: G::Shrinker<'value_gen>,
    made_progress_this_round: bool,
    made_progress_at_all: bool,
}

impl<'value_gen, G: ValueGen> ShrinkElements<'value_gen, G> {
    pub fn new(value_gen: &'value_gen G, simplest_known_failing: Box<[G::Seed]>) -> Self {
        // TODO: document panic condition
        assert!(!simplest_known_failing.is_empty());

        let index = 0;
        let shrinker = value_gen.new_shrinker(simplest_known_failing[index].clone());
        let mut this = Self {
            value_gen,
            simplest_known_failing,
            index,
            elem_shrinker: shrinker,
            made_progress_this_round: false,
            made_progress_at_all: false,
        };

        this.progress_if_shrinker_done();

        this
    }

    fn progress_if_shrinker_done(&mut self) {
        while self.index < self.simplest_known_failing.len()
            && self.elem_shrinker.current_attempt().is_none()
        {
            self.index += 1;

            // If we've gone past the last index, we're done
            if self.index == self.simplest_known_failing.len() {
                break;
            }

            self.elem_shrinker = self
                .value_gen
                .new_shrinker(self.simplest_known_failing[self.index].clone());
        }
    }
}

impl<'value_gen, G: ValueGen> ShrinkElements<'value_gen, G> {
    pub fn current_attempt(&self) -> Option<Box<[G::Seed]>> {
        (self.index < self.simplest_known_failing.len()).then(|| {
            let mut attempt = self.simplest_known_failing.clone();
            attempt[self.index] = self
                .elem_shrinker
                .current_attempt()
                .expect("element shrinker was done, but we aren't");
            attempt
        })
    }

    pub fn update(mut self, current_attempt_passed: bool) -> VecShrinker<'value_gen, G> {
        assert!(self.index < self.simplest_known_failing.len());

        // If the current attempt passed, update our element shrinker and carry on
        if current_attempt_passed {
            // Update the element shrinker
            self.elem_shrinker.update(true);
            self.progress_if_shrinker_done();

            // If this shrinking step is done, move on to the next step
            if self.index == self.simplest_known_failing.len() {
                // If we've made progress in shrinking some elements this round, try again from the
                // start in case shrinking later elements has made it possible to shrink earlier
                // elements
                if self.made_progress_this_round {
                    VecShrinker::ShrinkElements(Self::new(
                        self.value_gen,
                        self.simplest_known_failing,
                    ))
                }
                // If we've made progress at all shrinking elements, go back to trying to shrink the
                // length in case shrinking elements has made it possible to shrink the length
                else if self.made_progress_at_all {
                    // If we haven't made progress shrinking any elements, move on to TODO
                    todo!()
                }
                // If we didn't make any progress shrinking any elements, we're done
                else {
                    VecShrinker::Done(Done::new())
                }
            }
            // If this shrinking step is not done, carry on with it
            else {
                VecShrinker::ShrinkElements(self)
            }
        }
        // If the current attempt failed, we have a new simplest known failing
        else {
            // Update the simplest known failing with our new shrunk element
            let mut simplest_known_failing = self.simplest_known_failing;
            simplest_known_failing[self.index] = self
                .elem_shrinker
                .current_attempt()
                .expect("element shrinker was done, but we aren't");

            // Update the shrinker (will be progressed further after constructing Self below)
            let mut elem_shrinker = self.elem_shrinker;
            elem_shrinker.update(false);

            // Construct a new (and updated) `Self`, also ensuring we handle the element shrinker
            // potentially being done after our above update
            let mut new = Self {
                value_gen: self.value_gen,
                simplest_known_failing,
                index: self.index,
                elem_shrinker,
                made_progress_this_round: true,
                made_progress_at_all: true,
            };
            new.progress_if_shrinker_done();

            VecShrinker::ShrinkElements(new)
        }
    }

    pub fn into_observations(self) -> Vec<crate::report::Observation> {
        vec![]
    }
}
