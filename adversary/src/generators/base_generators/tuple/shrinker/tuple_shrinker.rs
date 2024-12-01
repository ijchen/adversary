use crate::{report::Observation, shrinker::Shrinker, InputGenerator};

use super::{all_together::*, elementwise::*, pairwise::*};

macro_rules! tuple_shrinker {
    ($(
        $n:literal { $($letter:ident)+ }
    )*) => {paste::paste!{$(
        // NOTE(ichen): Implementations for tuples of arity 0, 1, and 2 should
        // be manual - this macro's implementation will overcomplicate in
        // meaningful ways.
        const _: () = assert!($n > 2);

        pub struct [<TupleShrinker $n>]<'gens, $([<Gen $letter>]: InputGenerator),+> {
            generators: ($(&'gens [<Gen $letter>]),+),
            current_values: ($([<Gen $letter>]::InputSource),+),
            phase: [<Phase $n>]<'gens, $([<Gen $letter>]),+>,
        }

        enum [<Phase $n>]<'gens, $([<Gen $letter>]: InputGenerator),+> {
            ElementWiseFirstPass([<Elementwise $n>]<'gens, $([<Gen $letter>]),+>),
            AllTogetherFirstPass([<AllTogether $n>]<'gens, $([<Gen $letter>]),+>),
            Pairwise([<Pairwise $n>]<'gens, $([<Gen $letter>]),+>),
            ElementWiseSecondPass([<Elementwise $n>]<'gens, $([<Gen $letter>]),+>),
            AllTogetherSecondPass([<AllTogether $n>]<'gens, $([<Gen $letter>]),+>),
            Done,
        }

        impl<'gens, $([<Gen $letter>]: InputGenerator),+>
            [<TupleShrinker $n>]<'gens, $([<Gen $letter>]),+>
        {
            pub fn new(
                generators: ($(&'gens [<Gen $letter>]),+),
                current_values: ($([<Gen $letter>]::InputSource),+),
            ) -> Self {
                let phase =
                    [<Phase $n>]::ElementWiseFirstPass([<Elementwise $n>]::new(generators, current_values.clone()));

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
                if let [<Phase $n>]::ElementWiseFirstPass(phase) = &self.phase {
                    if phase.is_done() {
                        self.phase = [<Phase $n>]::AllTogetherFirstPass([<AllTogether $n>]::new(
                            self.generators,
                            self.current_values.clone(),
                        ));
                    }
                }

                // If AllTogetherFirstPass is done, progress to Pairwise
                if let [<Phase $n>]::AllTogetherFirstPass(phase) = &self.phase {
                    if phase.is_done() {
                        self.phase =
                            [<Phase $n>]::Pairwise([<Pairwise $n>]::new(self.generators, self.current_values.clone()));
                    }
                }

                // If Pairwise is done, progress to ElementWiseSecondPass
                if let [<Phase $n>]::Pairwise(phase) = &self.phase {
                    if phase.is_done() {
                        self.phase = [<Phase $n>]::ElementWiseSecondPass([<Elementwise $n>]::new(
                            self.generators,
                            self.current_values.clone(),
                        ));
                    }
                }

                // If ElementWiseSecondPass is done, progress to AllTogetherSecondPass
                if let [<Phase $n>]::ElementWiseSecondPass(phase) = &self.phase {
                    if phase.is_done() {
                        self.phase = [<Phase $n>]::AllTogetherSecondPass([<AllTogether $n>]::new(
                            self.generators,
                            self.current_values.clone(),
                        ));
                    }
                }

                // If AllTogetherSecondPass is done, progress to Done
                if let [<Phase $n>]::AllTogetherSecondPass(phase) = &self.phase {
                    if phase.is_done() {
                        self.phase = [<Phase $n>]::Done;
                    }
                }
            }
        }

        impl<'gens, $([<Gen $letter>]: InputGenerator),+> Shrinker
            for [<TupleShrinker $n>]<'gens, $([<Gen $letter>]),+>
        {
            type InputSource = ($([<Gen $letter>]::InputSource),+);

            fn current_attempt(&self) -> Option<Self::InputSource> {
                match &self.phase {
                    [<Phase $n>]::ElementWiseFirstPass(phase) => phase.current_attempt(),
                    [<Phase $n>]::AllTogetherFirstPass(phase) => phase.current_attempt(),
                    [<Phase $n>]::Pairwise(phase) => phase.current_attempt(),
                    [<Phase $n>]::ElementWiseSecondPass(phase) => phase.current_attempt(),
                    [<Phase $n>]::AllTogetherSecondPass(phase) => phase.current_attempt(),
                    [<Phase $n>]::Done => None,
                }
            }

            fn update(&mut self, current_attempt_passed: bool) {
                if !current_attempt_passed {
                    self.current_values = self.current_attempt().unwrap();
                }

                match &mut self.phase {
                    [<Phase $n>]::ElementWiseFirstPass(phase) => {
                        phase.update(self.generators, current_attempt_passed)
                    }
                    [<Phase $n>]::AllTogetherFirstPass(phase) => phase.update(current_attempt_passed),
                    [<Phase $n>]::Pairwise(phase) => phase.update(self.generators, current_attempt_passed),
                    [<Phase $n>]::ElementWiseSecondPass(phase) => {
                        phase.update(self.generators, current_attempt_passed)
                    }
                    [<Phase $n>]::AllTogetherSecondPass(phase) => phase.update(current_attempt_passed),
                    [<Phase $n>]::Done => { /* Nothing to do here */ }
                }
            }

            fn into_observations(self) -> Vec<Observation> {
                // TODO: useful observations
                Vec::new()
            }
        }
    )*}};
}

tuple_shrinker! {
    3 { A B C }
    4 { A B C D }
    5 { A B C D E }
    6 { A B C D E F }
    7 { A B C D E F G }
    8 { A B C D E F G H }
    9 { A B C D E F G H I }
    10 { A B C D E F G H I J }
    11 { A B C D E F G H I J K }
    12 { A B C D E F G H I J K L }
}
