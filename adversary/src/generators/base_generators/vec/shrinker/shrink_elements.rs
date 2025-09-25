use crate::{
    RangeAwareValueGen, ValueGen,
    shrinker::Shrinker as _,
    vec::shrinker::{Done, Pairs, RemoveElems, SingleElems, Subsets, VecShrinker},
};

#[derive(Debug)]
pub struct ShrinkElements<'gens, G: ValueGen, L: RangeAwareValueGen<Value = usize>> {
    value_gen: &'gens G,
    len_gen: &'gens L,
    simplest_known_failing: Box<[G::Seed]>,
    index: usize, // index == simplest_known_failing.len() indicates we're done shrinking
    // Invariant: if self isn't done shrinking (index < simplest_known_failing.len()), neither is
    // elem_shrinker
    elem_shrinker: G::Shrinker<'gens>,
    made_progress_this_round: bool,
    made_progress_at_all: bool,
}

impl<'gens, G: ValueGen, L: RangeAwareValueGen<Value = usize>> ShrinkElements<'gens, G, L> {
    /// Returns a new [`ShrinkElements`], or [`Err`] with `simplest_known_failing` if this step
    /// should be skipped.
    pub fn new(
        value_gen: &'gens G,
        len_gen: &'gens L,
        simplest_known_failing: Box<[G::Seed]>,
    ) -> Result<Self, Box<[G::Seed]>> {
        // If the seed is empty, there are no elements to shrink
        if simplest_known_failing.is_empty() {
            return Err(simplest_known_failing);
        }

        let index = 0;
        let shrinker = value_gen.new_shrinker(simplest_known_failing[index].clone());
        let mut this = Self {
            value_gen,
            len_gen,
            simplest_known_failing,
            index,
            elem_shrinker: shrinker,
            made_progress_this_round: false,
            made_progress_at_all: false,
        };

        this.progress_if_shrinker_done();

        // If no element shrinker could make progress, we have no work to do
        if this.index == this.simplest_known_failing.len() {
            return Err(this.simplest_known_failing);
        }

        Ok(this)
    }

    fn new_with_past_progress(
        value_gen: &'gens G,
        len_gen: &'gens L,
        simplest_known_failing: Box<[G::Seed]>,
    ) -> Result<Self, Box<[G::Seed]>> {
        let mut this = Self::new(value_gen, len_gen, simplest_known_failing)?;
        this.made_progress_at_all = true;
        Ok(this)
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

impl<'gens, G: ValueGen, L: RangeAwareValueGen<Value = usize>> ShrinkElements<'gens, G, L> {
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

    pub fn update(mut self, current_attempt_passed: bool) -> VecShrinker<'gens, G, L> {
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
                let simplest_known_failing = if self.made_progress_this_round {
                    match Self::new_with_past_progress(
                        self.value_gen,
                        self.len_gen,
                        self.simplest_known_failing,
                    ) {
                        Ok(shrink_elements) => return VecShrinker::ShrinkElements(shrink_elements),
                        Err(simplest_known_failing) => simplest_known_failing,
                    }
                } else {
                    self.simplest_known_failing
                };

                // If we've made progress at all shrinking elements, go back to trying to shrink the
                // length in case shrinking elements has made it possible to shrink the length
                if self.made_progress_at_all {
                    // Try `SingleElems`
                    let simplest_known_failing = match SingleElems::new(
                        self.value_gen,
                        self.len_gen,
                        simplest_known_failing,
                    ) {
                        Ok(single_elems) => return VecShrinker::SingleElems(single_elems),
                        Err(simplest_known_failing) => simplest_known_failing,
                    };

                    // `SingleElems` had to be skipped, try `Pairs`
                    let simplest_known_failing =
                        match Pairs::new(self.value_gen, self.len_gen, simplest_known_failing) {
                            Ok(pairs) => return VecShrinker::Pairs(pairs),
                            Err(simplest_known_failing) => simplest_known_failing,
                        };

                    // `Pairs` had to be skipped, try `RemoveElems`
                    let simplest_known_failing = match RemoveElems::new(
                        self.value_gen,
                        self.len_gen,
                        simplest_known_failing,
                        false,
                    ) {
                        Ok(remove_elems) => return VecShrinker::RemoveElems(remove_elems),
                        Err(simplest_known_failing) => simplest_known_failing,
                    };

                    // `RemoveElems` had to be skipped, try `Subsets`
                    if let Ok(subsets) =
                        Subsets::new(self.value_gen, self.len_gen, simplest_known_failing, false)
                    {
                        return VecShrinker::Subsets(subsets);
                    };

                    // All length-shrinking steps had to be skipped, we're done
                    VecShrinker::Done(Done::new())
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
            self.made_progress_this_round = true;
            self.made_progress_at_all = true;

            // Update the simplest known failing with our new shrunk element
            let mut simplest_known_failing = self.simplest_known_failing;
            simplest_known_failing[self.index] = self
                .elem_shrinker
                .current_attempt()
                .expect("element shrinker was done, but we aren't");
            self.simplest_known_failing = simplest_known_failing;

            // Update the shrinker
            self.elem_shrinker.update(false);
            self.progress_if_shrinker_done();

            VecShrinker::ShrinkElements(self)
        }
    }

    pub fn into_observations(self) -> Vec<crate::report::Observation> {
        vec![]
    }
}
