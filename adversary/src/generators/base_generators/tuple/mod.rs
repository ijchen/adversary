mod binary;
mod cartesian_product;
mod pair;
mod shrinker;
mod unary;

use crate::{IntoValueGen, ValueGen};

pub use pair::Pair;

macro_rules! impl_tuple_into_value_gen {
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

        impl<
            $($letter),+,
            $([<IntoGen $letter>]: IntoValueGen<[<$letter>]>),+,
        > IntoValueGen<($([<$letter>]),+)> for ($([<IntoGen $letter>]),+)
        {
            // TODO: use ATPIT once stabilized
            type Gen = [<TupleGen $n>]<$([<IntoGen $letter>]::Gen),+>;

            fn into_value_gen(self) -> Self::Gen {
                [<TupleGen $n>](
                    $(self.[<$index>].into_value_gen()),+
                )
            }
        }

        // TODO: this should not be pub, make private once ATPIT allows
        // IntoValueGen impls to hide the concrete type of IntoValueGen::Gen
        pub struct [<TupleGen $n>]<$([<Gen $letter>]),+>($([<Gen $letter>]),+);

        impl<$([<Gen $letter>]: ValueGen),+> ValueGen
            for [<TupleGen $n>]<$([<Gen $letter>]),+>
        where
            $([<Gen $letter>]::Shrinker: 'static,)+
        {
            type Value = ($([<Gen $letter>]::Value),+);
            type Seed = ($([<Gen $letter>]::Seed),+);
            // TODO: use ATPIT once stabilized
            type Shrinker = shrinker::[<TupleShrinker $n>]<$([<Gen $letter>]),+>;

            #[expect(
                clippy::needless_question_mark,
                reason = "this code pattern is easier to generate in a macro"
            )]
            fn cardinality(&self) -> Option<usize> {
                Some(1usize $(.checked_mul(self.$index.cardinality()?)?)+)
            }

            fn exhaustive(&self) -> impl Iterator<Item = Self::Seed> {
                cartesian_product::[<cartesian_product_ $n>]($(|| self.$index.exhaustive()),+)
            }

            #[expect(
                clippy::needless_question_mark,
                reason = "this code pattern is easier to generate in a macro"
            )]
            fn adversarial_count(&self) -> Option<usize> {
                Some(1usize $(.checked_mul(self.$index.adversarial_count()?)?)+)
            }

            fn adversarial(&self) -> impl Iterator<Item = Self::Seed> {
                cartesian_product::[<cartesian_product_ $n>]($(|| self.$index.adversarial()),+)
            }

            fn sample(&self, rng: &mut (impl rand::Rng + ?Sized)) -> Self::Seed {
                ($(self.$index.sample(rng)),+)
            }

            fn new_shrinker(
                &self,
                failing_value_seed: Self::Seed,
            ) -> Self::Shrinker {
                shrinker::[<TupleShrinker $n>]::new(($(&self.$index),+), failing_value_seed)
            }

            fn create_value(&self, seed: Self::Seed) -> Self::Value {
                ($(self.$index.create_value(seed.$index)),+)
            }
        }
    )*}};
}

impl_tuple_into_value_gen! {
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
