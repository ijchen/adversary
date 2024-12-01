mod cartesian_product;
mod shrinker;
mod unary;

use crate::{shrinker::Shrinker, InputGenerator, IntoInputGenerator};

macro_rules! impl_tuple_into_input_generator {
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
            $([<IntoGen $letter>]: IntoInputGenerator<[<$letter>]>),+,
        > IntoInputGenerator<($([<$letter>]),+)> for ($([<IntoGen $letter>]),+)
        {
            fn into_input_generator(self) -> impl InputGenerator<Input = ($([<$letter>]),+)> {
                [<TupleGen $n>](
                    $(self.[<$index>].into_input_generator()),+
                )
            }
        }

        struct [<TupleGen $n>]<$([<Gen $letter>]),+>($([<Gen $letter>]),+);

        impl<$([<Gen $letter>]: InputGenerator),+> InputGenerator
            for [<TupleGen $n>]<$([<Gen $letter>]),+>
        {
            type Input = ($([<Gen $letter>]::Input),+);

            type InputSource = ($([<Gen $letter>]::InputSource),+);

            #[expect(
                clippy::needless_question_mark,
                reason = "this code pattern is easier to generate in a macro"
            )]
            fn cardinality(&self) -> Option<usize> {
                Some(1usize $(.checked_mul(self.$index.cardinality()?)?)+)
            }

            fn exhaustive(&self) -> impl Iterator<Item = Self::InputSource> {
                cartesian_product::[<cartesian_product_ $n>]($(|| self.$index.exhaustive()),+)
            }

            #[expect(
                clippy::needless_question_mark,
                reason = "this code pattern is easier to generate in a macro"
            )]
            fn adversarial_count(&self) -> Option<usize> {
                Some(1usize $(.checked_mul(self.$index.adversarial_count()?)?)+)
            }

            fn adversarial(&self) -> impl Iterator<Item = Self::InputSource> {
                cartesian_product::[<cartesian_product_ $n>]($(|| self.$index.adversarial()),+)
            }

            fn sample(&self, rng: &mut (impl rand::Rng + ?Sized)) -> Self::InputSource {
                ($(self.$index.sample(rng)),+)
            }

            fn new_shrinker(
                &self,
                failing_input: Self::InputSource,
            ) -> impl Shrinker<InputSource = Self::InputSource> {
                shrinker::[<TupleShrinker $n>]::new(($(&self.$index),+), failing_input)
            }

            fn create_input(&self, input_source: Self::InputSource) -> Self::Input {
                ($(self.$index.create_input(input_source.$index)),+)
            }
        }
    )*}};
}

impl_tuple_into_input_generator! {
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
