use crate::{
    generators::base_generators::tuple::*, report::Observation, shrinker::Shrinker, ValueGen,
};

use super::{all_together::*, elementwise::*, pairwise::*};

macro_rules! tuple_shrinker {
    ($(
        $n:literal { $($letter:ident)+ }
    )*) => {paste::paste!{$(
        // NOTE(ichen): Implementations for tuples of arity 0, 1, and 2 should
        // be manual - this macro's implementation will overcomplicate in
        // meaningful ways.
        const _: () = assert!($n > 2);

        pub struct [<TupleShrinker $n>]<$([<Gen $letter>]: ValueGen),+> {
            current_values: ($([<Gen $letter>]::Seed),+),
            phase: [<Phase $n>]<$([<Gen $letter>]),+>,
        }

        enum [<Phase $n>]<$([<Gen $letter>]: ValueGen),+> {
            ElementwiseFirstPass([<Elementwise $n>]<$([<Gen $letter>]),+>),
            AllTogetherFirstPass([<AllTogether $n>]<$([<Gen $letter>]),+>),
            Pairwise([<Pairwise $n>]<$([<Gen $letter>]),+>),
            ElementwiseSecondPass([<Elementwise $n>]<$([<Gen $letter>]),+>),
            AllTogetherSecondPass([<AllTogether $n>]<$([<Gen $letter>]),+>),
            Done,
        }

        impl<$([<Gen $letter>]: ValueGen),+> [<TupleShrinker $n>]<$([<Gen $letter>]),+>
        where
            $([<Gen $letter>]::Shrinker: 'static,)+
        {
            pub fn new(
                generator: &[<TupleGen $n>]<$([<Gen $letter>]),+>,
                current_values: ($([<Gen $letter>]::Seed),+),
            ) -> Self {
                let phase = [<Phase $n>]::ElementwiseFirstPass([<Elementwise $n>]::new(generator, current_values.clone()));

                let mut this = Self {
                    current_values,
                    phase,
                };

                this.progress_if_necessary(generator);

                this
            }

            pub fn progress_if_necessary(
                &mut self,
                generator: &[<TupleGen $n>]<$([<Gen $letter>]),+>,
            ) {
                // If ElementwiseFirstPass is done, progress to AllTogetherFirstPass
                if let [<Phase $n>]::ElementwiseFirstPass(phase) = &self.phase {
                    if phase.is_done() {
                        self.phase = [<Phase $n>]::AllTogetherFirstPass([<AllTogether $n>]::new(
                            generator,
                            self.current_values.clone(),
                        ));
                    }
                }

                // If AllTogetherFirstPass is done, progress to Pairwise
                if let [<Phase $n>]::AllTogetherFirstPass(phase) = &self.phase {
                    if phase.is_done() {
                        self.phase = [<Phase $n>]::Pairwise([<Pairwise $n>]::new(
                            generator,
                            self.current_values.clone()
                        ));
                    }
                }

                // If Pairwise is done, progress to ElementwiseSecondPass
                if let [<Phase $n>]::Pairwise(phase) = &self.phase {
                    if phase.is_done() {
                        self.phase = [<Phase $n>]::ElementwiseSecondPass([<Elementwise $n>]::new(
                            generator,
                            self.current_values.clone(),
                        ));
                    }
                }

                // If ElementwiseSecondPass is done, progress to AllTogetherSecondPass
                if let [<Phase $n>]::ElementwiseSecondPass(phase) = &self.phase {
                    if phase.is_done() {
                        self.phase = [<Phase $n>]::AllTogetherSecondPass([<AllTogether $n>]::new(
                            generator,
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

        impl<$([<Gen $letter>]: ValueGen),+> Shrinker<[<TupleGen $n>]<$([<Gen $letter>]),+>>
            for [<TupleShrinker $n>]<$([<Gen $letter>]),+>
        where
            $([<Gen $letter>]::Shrinker: 'static,)+
        {
            fn current_attempt(&self) -> Option<<[<TupleGen $n>]<$([<Gen $letter>]),+> as ValueGen>::Seed> {
                match &self.phase {
                    [<Phase $n>]::ElementwiseFirstPass(phase) => phase.current_attempt(),
                    [<Phase $n>]::AllTogetherFirstPass(phase) => phase.current_attempt(),
                    [<Phase $n>]::Pairwise(phase) => phase.current_attempt(),
                    [<Phase $n>]::ElementwiseSecondPass(phase) => phase.current_attempt(),
                    [<Phase $n>]::AllTogetherSecondPass(phase) => phase.current_attempt(),
                    [<Phase $n>]::Done => None,
                }
            }

            fn update(&mut self, generator: &[<TupleGen $n>]<$([<Gen $letter>]),+>, current_attempt_passed: bool) {
                if !current_attempt_passed {
                    self.current_values = self.current_attempt().unwrap();
                }

                match &mut self.phase {
                    [<Phase $n>]::ElementwiseFirstPass(phase) => phase.update(generator, current_attempt_passed),
                    [<Phase $n>]::AllTogetherFirstPass(phase) => phase.update(generator, current_attempt_passed),
                    [<Phase $n>]::Pairwise(phase) => phase.update(generator, current_attempt_passed),
                    [<Phase $n>]::ElementwiseSecondPass(phase) => phase.update(generator, current_attempt_passed),
                    [<Phase $n>]::AllTogetherSecondPass(phase) => phase.update(generator, current_attempt_passed),
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
