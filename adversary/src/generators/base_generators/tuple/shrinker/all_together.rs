use crate::{generators::base_generators::tuple::*, shrinker::Shrinker, ValueGen};

macro_rules! all_together {
    ($(
        $n:literal {
            @ letters : $($letter:ident)+
            @ indices : $($index:literal)+
        }
    )*) => {paste::paste!{$(
        // NOTE(ichen): Implementations for tuples of arity 0, 1, and 2 should
        // be manual - this macro's implementation will overcomplicate in
        // meaningful ways.
        const _: () = assert!($n > 2);

        pub struct [<AllTogether $n>]<$([<Gen $letter>]: ValueGen),+> {
            current_values: ($([<Gen $letter>]::Seed),+),
            shrinkers: ($(Box<dyn Shrinker<[<Gen $letter>]>>,)+),
        }

        impl<$([<Gen $letter>]: ValueGen),+> [<AllTogether $n>]<$([<Gen $letter>]),+> {
            pub fn new(
                generator: &[<TupleGen $n>]<$([<Gen $letter>]),+>,
                current_values: ($([<Gen $letter>]::Seed),+),
            ) -> Self
            where
                $([<Gen $letter>]::Shrinker: 'static,)+
            {
                let shrinkers = ($(
                    Box::new(generator.$index.new_shrinker(current_values.$index.clone())) as _,
                )+);

                Self {
                    current_values,
                    shrinkers,
                }
            }

            pub fn is_done(&self) -> bool {
                $(self.shrinkers.$index.current_attempt().is_none())&&+
            }

            pub fn current_attempt(
                &self,
            ) -> Option<($([<Gen $letter>]::Seed),+)> {
                match ($(self.shrinkers.$index.current_attempt()),+) {
                    // If all shrinkers are done, so are we
                    ($(Option::<[<Gen $letter>]::Seed>::None),+) => None,

                    // As long as any shrinker can make progress, keep trying
                    attempts => Some((
                        $(attempts.$index.unwrap_or_else(|| self.current_values.$index.clone()),)+
                    )),
                }
            }

            pub fn update(
                &mut self,
                generator: &[<TupleGen $n>]<$([<Gen $letter>]),+>,
                current_attempt_passed: bool,
            ) {
                match ($(self.shrinkers.$index.current_attempt()),+) {
                    ($(Option::<[<Gen $letter>]::Seed>::None),+) => {
                        panic!(concat!("`AllTogether", $n, "::update` called while all shrinkers were done"))
                    }

                    // TODO(ichen): I think there's some kinda weird implications here
                    // where we're telling shrinkers that an attempt passed or failed,
                    // but that passing or failure may have nothing to do with their
                    // attempt. I could see this leading to weird or even problematic
                    // behavior... should investigate.
                    attempts => {
                        $(
                            if attempts.$index.is_some() {
                                self.shrinkers.$index.update(&generator.$index, current_attempt_passed);
                            }
                        )+
                    }
                }
            }
        }
    )*}};
}

all_together! {
    3 {
        @letters: A B C
        @indices: 0 1 2
    }
    4 {
        @letters: A B C D
        @indices: 0 1 2 3
    }
    5 {
        @letters: A B C D E
        @indices: 0 1 2 3 4
    }
    6 {
        @letters: A B C D E F
        @indices: 0 1 2 3 4 5
    }
    7 {
        @letters: A B C D E F G
        @indices: 0 1 2 3 4 5 6
    }
    8 {
        @letters: A B C D E F G H
        @indices: 0 1 2 3 4 5 6 7
    }
    9 {
        @letters: A B C D E F G H I
        @indices: 0 1 2 3 4 5 6 7 8
    }
    10 {
        @letters: A B C D E F G H I J
        @indices: 0 1 2 3 4 5 6 7 8 9
    }
    11 {
        @letters: A B C D E F G H I J K
        @indices: 0 1 2 3 4 5 6 7 8 9 10
    }
    12 {
        @letters: A B C D E F G H I J K L
        @indices: 0 1 2 3 4 5 6 7 8 9 10 11
    }
}
