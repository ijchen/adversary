use crate::{generators::base_generators::tuple::*, ValueGen};

use super::super::Pair;

macro_rules! pairwise {
    ($(
        $n:literal {
            @ letters : $($letter:ident)+
            @ shrinking_helper :
                $( (
                    $shrinking_first_letter:ident
                    $shrinking_first_index:literal
                    $shrinking_second_letter:ident
                    $shrinking_second_index:literal
                    |
                    $($shrinking_section_left:literal)*
                    #
                    $($shrinking_section_center:literal)*
                    #
                    $($shrinking_section_right:literal)*
                    =>
                    $(
                        $shrinking_to_first_letter:ident
                        $shrinking_to_first_index:literal
                        $shrinking_to_second_letter:ident
                        $shrinking_to_second_index:literal
                    )?
                    $( @ $shrinking_unit:expr )?
                ) )+
        }
    )*) => {paste::paste!{$(
        // NOTE(ichen): Implementations for tuples of arity 0, 1, and 2 should
        // be manual - this macro's implementation will overcomplicate in
        // meaningful ways.
        const _: () = assert!($n > 2);

        pub struct [<Pairwise $n>]<$([<Gen $letter>]: ValueGen),+> {
            current_values: ($([<Gen $letter>]::Seed),+),
            step: [<Step $n>]<$([<Gen $letter>]),+>,
        }

        // TODO(ichen): is there a nicer way to do this than an enum for every possible
        // combination of two generators?
        enum [<Step $n>]<$([<Gen $letter>]: ValueGen),+> {
            $([<Shrinking $shrinking_first_letter $shrinking_second_letter>](
                Pair<[<Gen $shrinking_first_letter>], [<Gen $shrinking_second_letter>]>
            ),)+
            Done,
        }

        impl<$([<Gen $letter>]: ValueGen),+> [<Pairwise $n>]<$([<Gen $letter>]),+>
        where
            $([<Gen $letter>]::Shrinker: 'static,)+
        {
            pub fn new(
                generator: &[<TupleGen $n>]<$([<Gen $letter>]),+>,
                current_values: ($([<Gen $letter>]::Seed),+),
            ) -> Self {
                let step = [<Step $n>]::ShrinkingAB(Pair::new(
                    generator.0.new_shrinker(current_values.0.clone()),
                    generator.1.new_shrinker(current_values.1.clone()),
                    current_values.0.clone(),
                    current_values.1.clone(),
                ));

                let mut this = Self {
                    current_values,
                    step,
                };

                this.progress_if_necessary(generator);

                this
            }

            pub fn progress_if_necessary(&mut self, generator: &[<TupleGen $n>]<$([<Gen $letter>]),+>) {
                // Probably the most sauced macro by example I've ever written
                $(
                    $(
                        // If a pair is done, progress to the next pair
                        if let [<Step $n>]::[<Shrinking $shrinking_first_letter $shrinking_second_letter>](pair) = &mut self.step {
                            if pair.is_done() {
                                self.step = [<Step $n>]::[<Shrinking $shrinking_to_first_letter $shrinking_to_second_letter>](Pair::new(
                                    generator.$shrinking_to_first_index.new_shrinker(self.current_values.$shrinking_to_first_index.clone()),
                                    generator.$shrinking_to_second_index.new_shrinker(self.current_values.$shrinking_to_second_index.clone()),
                                    self.current_values.$shrinking_to_first_index.clone(),
                                    self.current_values.$shrinking_to_second_index.clone(),
                                ));
                            }
                        }
                    )?
                    $(
                        #[expect(
                            clippy::no_effect,
                            reason = "we need to repeat something at this depth so Rust knows how many times to repeat this repetition"
                        )]
                        $shrinking_unit;

                        // If the last pair is done, progress to Done
                        if let [<Step $n>]::[<Shrinking $shrinking_first_letter $shrinking_second_letter>](pair) = &mut self.step {
                            if pair.is_done() {
                                self.step = [<Step $n>]::Done;
                            }
                        }
                    )?
                )+
            }

            pub fn is_done(&self) -> bool {
                matches!(self.step, [<Step $n>]::Done)
            }

            pub fn current_attempt(
                &self,
            ) -> Option<($([<Gen $letter>]::Seed),+)> {
                match &self.step {
                    $(
                        [<Step $n>]::[<Shrinking $shrinking_first_letter $shrinking_second_letter>](pair) => {
                            let (first, second) = pair.current_attempt().unwrap();
                            Some((
                                $(self.current_values.$shrinking_section_left.clone(),)*
                                first,
                                $(self.current_values.$shrinking_section_center.clone(),)*
                                second,
                                $(self.current_values.$shrinking_section_right.clone(),)*
                            ))
                        }
                    )+
                    [<Step $n>]::Done => None,
                }
            }

            pub fn update(
                &mut self,
                generator: &[<TupleGen $n>]<$([<Gen $letter>]),+>,
                current_attempt_passed: bool,
            ) {
                match &mut self.step {
                    $([<Step $n>]::[<Shrinking $shrinking_first_letter $shrinking_second_letter>](pair) => {
                        pair.update((&generator.$shrinking_first_index, &generator.$shrinking_second_index), current_attempt_passed)
                    })+
                    [<Step $n>]::Done => panic!(concat!("`Elementwise", $n, "::update` called while in `Step::Done`")),
                }

                self.progress_if_necessary(generator);
            }
        }
    )*}};
}

// Macro invocation code generated with:
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=c2aa23a36c91013322f67850eeb76bd8
pairwise! {
    3 {
        @letters: A B C
        @shrinking_helper:
            (A 0 B 1 | # # 2 => A 0 C 2)
            (A 0 C 2 | # 1 # => B 1 C 2)
            (B 1 C 2 | 0 # # => @())
    }
    4 {
        @letters: A B C D
        @shrinking_helper:
            (A 0 B 1 | # # 2 3 => A 0 C 2)
            (A 0 C 2 | # 1 # 3 => A 0 D 3)
            (A 0 D 3 | # 1 2 # => B 1 C 2)
            (B 1 C 2 | 0 # # 3 => B 1 D 3)
            (B 1 D 3 | 0 # 2 # => C 2 D 3)
            (C 2 D 3 | 0 1 # # => @())
    }
    5 {
        @letters: A B C D E
        @shrinking_helper:
            (A 0 B 1 | # # 2 3 4 => A 0 C 2)
            (A 0 C 2 | # 1 # 3 4 => A 0 D 3)
            (A 0 D 3 | # 1 2 # 4 => A 0 E 4)
            (A 0 E 4 | # 1 2 3 # => B 1 C 2)
            (B 1 C 2 | 0 # # 3 4 => B 1 D 3)
            (B 1 D 3 | 0 # 2 # 4 => B 1 E 4)
            (B 1 E 4 | 0 # 2 3 # => C 2 D 3)
            (C 2 D 3 | 0 1 # # 4 => C 2 E 4)
            (C 2 E 4 | 0 1 # 3 # => D 3 E 4)
            (D 3 E 4 | 0 1 2 # # => @())
    }
    6 {
        @letters: A B C D E F
        @shrinking_helper:
            (A 0 B 1 | # # 2 3 4 5 => A 0 C 2)
            (A 0 C 2 | # 1 # 3 4 5 => A 0 D 3)
            (A 0 D 3 | # 1 2 # 4 5 => A 0 E 4)
            (A 0 E 4 | # 1 2 3 # 5 => A 0 F 5)
            (A 0 F 5 | # 1 2 3 4 # => B 1 C 2)
            (B 1 C 2 | 0 # # 3 4 5 => B 1 D 3)
            (B 1 D 3 | 0 # 2 # 4 5 => B 1 E 4)
            (B 1 E 4 | 0 # 2 3 # 5 => B 1 F 5)
            (B 1 F 5 | 0 # 2 3 4 # => C 2 D 3)
            (C 2 D 3 | 0 1 # # 4 5 => C 2 E 4)
            (C 2 E 4 | 0 1 # 3 # 5 => C 2 F 5)
            (C 2 F 5 | 0 1 # 3 4 # => D 3 E 4)
            (D 3 E 4 | 0 1 2 # # 5 => D 3 F 5)
            (D 3 F 5 | 0 1 2 # 4 # => E 4 F 5)
            (E 4 F 5 | 0 1 2 3 # # => @())
    }
    7 {
        @letters: A B C D E F G
        @shrinking_helper:
            (A 0 B 1 | # # 2 3 4 5 6 => A 0 C 2)
            (A 0 C 2 | # 1 # 3 4 5 6 => A 0 D 3)
            (A 0 D 3 | # 1 2 # 4 5 6 => A 0 E 4)
            (A 0 E 4 | # 1 2 3 # 5 6 => A 0 F 5)
            (A 0 F 5 | # 1 2 3 4 # 6 => A 0 G 6)
            (A 0 G 6 | # 1 2 3 4 5 # => B 1 C 2)
            (B 1 C 2 | 0 # # 3 4 5 6 => B 1 D 3)
            (B 1 D 3 | 0 # 2 # 4 5 6 => B 1 E 4)
            (B 1 E 4 | 0 # 2 3 # 5 6 => B 1 F 5)
            (B 1 F 5 | 0 # 2 3 4 # 6 => B 1 G 6)
            (B 1 G 6 | 0 # 2 3 4 5 # => C 2 D 3)
            (C 2 D 3 | 0 1 # # 4 5 6 => C 2 E 4)
            (C 2 E 4 | 0 1 # 3 # 5 6 => C 2 F 5)
            (C 2 F 5 | 0 1 # 3 4 # 6 => C 2 G 6)
            (C 2 G 6 | 0 1 # 3 4 5 # => D 3 E 4)
            (D 3 E 4 | 0 1 2 # # 5 6 => D 3 F 5)
            (D 3 F 5 | 0 1 2 # 4 # 6 => D 3 G 6)
            (D 3 G 6 | 0 1 2 # 4 5 # => E 4 F 5)
            (E 4 F 5 | 0 1 2 3 # # 6 => E 4 G 6)
            (E 4 G 6 | 0 1 2 3 # 5 # => F 5 G 6)
            (F 5 G 6 | 0 1 2 3 4 # # => @())
    }
    8 {
        @letters: A B C D E F G H
        @shrinking_helper:
            (A 0 B 1 | # # 2 3 4 5 6 7 => A 0 C 2)
            (A 0 C 2 | # 1 # 3 4 5 6 7 => A 0 D 3)
            (A 0 D 3 | # 1 2 # 4 5 6 7 => A 0 E 4)
            (A 0 E 4 | # 1 2 3 # 5 6 7 => A 0 F 5)
            (A 0 F 5 | # 1 2 3 4 # 6 7 => A 0 G 6)
            (A 0 G 6 | # 1 2 3 4 5 # 7 => A 0 H 7)
            (A 0 H 7 | # 1 2 3 4 5 6 # => B 1 C 2)
            (B 1 C 2 | 0 # # 3 4 5 6 7 => B 1 D 3)
            (B 1 D 3 | 0 # 2 # 4 5 6 7 => B 1 E 4)
            (B 1 E 4 | 0 # 2 3 # 5 6 7 => B 1 F 5)
            (B 1 F 5 | 0 # 2 3 4 # 6 7 => B 1 G 6)
            (B 1 G 6 | 0 # 2 3 4 5 # 7 => B 1 H 7)
            (B 1 H 7 | 0 # 2 3 4 5 6 # => C 2 D 3)
            (C 2 D 3 | 0 1 # # 4 5 6 7 => C 2 E 4)
            (C 2 E 4 | 0 1 # 3 # 5 6 7 => C 2 F 5)
            (C 2 F 5 | 0 1 # 3 4 # 6 7 => C 2 G 6)
            (C 2 G 6 | 0 1 # 3 4 5 # 7 => C 2 H 7)
            (C 2 H 7 | 0 1 # 3 4 5 6 # => D 3 E 4)
            (D 3 E 4 | 0 1 2 # # 5 6 7 => D 3 F 5)
            (D 3 F 5 | 0 1 2 # 4 # 6 7 => D 3 G 6)
            (D 3 G 6 | 0 1 2 # 4 5 # 7 => D 3 H 7)
            (D 3 H 7 | 0 1 2 # 4 5 6 # => E 4 F 5)
            (E 4 F 5 | 0 1 2 3 # # 6 7 => E 4 G 6)
            (E 4 G 6 | 0 1 2 3 # 5 # 7 => E 4 H 7)
            (E 4 H 7 | 0 1 2 3 # 5 6 # => F 5 G 6)
            (F 5 G 6 | 0 1 2 3 4 # # 7 => F 5 H 7)
            (F 5 H 7 | 0 1 2 3 4 # 6 # => G 6 H 7)
            (G 6 H 7 | 0 1 2 3 4 5 # # => @())
    }
    9 {
        @letters: A B C D E F G H I
        @shrinking_helper:
            (A 0 B 1 | # # 2 3 4 5 6 7 8 => A 0 C 2)
            (A 0 C 2 | # 1 # 3 4 5 6 7 8 => A 0 D 3)
            (A 0 D 3 | # 1 2 # 4 5 6 7 8 => A 0 E 4)
            (A 0 E 4 | # 1 2 3 # 5 6 7 8 => A 0 F 5)
            (A 0 F 5 | # 1 2 3 4 # 6 7 8 => A 0 G 6)
            (A 0 G 6 | # 1 2 3 4 5 # 7 8 => A 0 H 7)
            (A 0 H 7 | # 1 2 3 4 5 6 # 8 => A 0 I 8)
            (A 0 I 8 | # 1 2 3 4 5 6 7 # => B 1 C 2)
            (B 1 C 2 | 0 # # 3 4 5 6 7 8 => B 1 D 3)
            (B 1 D 3 | 0 # 2 # 4 5 6 7 8 => B 1 E 4)
            (B 1 E 4 | 0 # 2 3 # 5 6 7 8 => B 1 F 5)
            (B 1 F 5 | 0 # 2 3 4 # 6 7 8 => B 1 G 6)
            (B 1 G 6 | 0 # 2 3 4 5 # 7 8 => B 1 H 7)
            (B 1 H 7 | 0 # 2 3 4 5 6 # 8 => B 1 I 8)
            (B 1 I 8 | 0 # 2 3 4 5 6 7 # => C 2 D 3)
            (C 2 D 3 | 0 1 # # 4 5 6 7 8 => C 2 E 4)
            (C 2 E 4 | 0 1 # 3 # 5 6 7 8 => C 2 F 5)
            (C 2 F 5 | 0 1 # 3 4 # 6 7 8 => C 2 G 6)
            (C 2 G 6 | 0 1 # 3 4 5 # 7 8 => C 2 H 7)
            (C 2 H 7 | 0 1 # 3 4 5 6 # 8 => C 2 I 8)
            (C 2 I 8 | 0 1 # 3 4 5 6 7 # => D 3 E 4)
            (D 3 E 4 | 0 1 2 # # 5 6 7 8 => D 3 F 5)
            (D 3 F 5 | 0 1 2 # 4 # 6 7 8 => D 3 G 6)
            (D 3 G 6 | 0 1 2 # 4 5 # 7 8 => D 3 H 7)
            (D 3 H 7 | 0 1 2 # 4 5 6 # 8 => D 3 I 8)
            (D 3 I 8 | 0 1 2 # 4 5 6 7 # => E 4 F 5)
            (E 4 F 5 | 0 1 2 3 # # 6 7 8 => E 4 G 6)
            (E 4 G 6 | 0 1 2 3 # 5 # 7 8 => E 4 H 7)
            (E 4 H 7 | 0 1 2 3 # 5 6 # 8 => E 4 I 8)
            (E 4 I 8 | 0 1 2 3 # 5 6 7 # => F 5 G 6)
            (F 5 G 6 | 0 1 2 3 4 # # 7 8 => F 5 H 7)
            (F 5 H 7 | 0 1 2 3 4 # 6 # 8 => F 5 I 8)
            (F 5 I 8 | 0 1 2 3 4 # 6 7 # => G 6 H 7)
            (G 6 H 7 | 0 1 2 3 4 5 # # 8 => G 6 I 8)
            (G 6 I 8 | 0 1 2 3 4 5 # 7 # => H 7 I 8)
            (H 7 I 8 | 0 1 2 3 4 5 6 # # => @())
    }
    10 {
        @letters: A B C D E F G H I J
        @shrinking_helper:
            (A 0 B 1 | # # 2 3 4 5 6 7 8 9 => A 0 C 2)
            (A 0 C 2 | # 1 # 3 4 5 6 7 8 9 => A 0 D 3)
            (A 0 D 3 | # 1 2 # 4 5 6 7 8 9 => A 0 E 4)
            (A 0 E 4 | # 1 2 3 # 5 6 7 8 9 => A 0 F 5)
            (A 0 F 5 | # 1 2 3 4 # 6 7 8 9 => A 0 G 6)
            (A 0 G 6 | # 1 2 3 4 5 # 7 8 9 => A 0 H 7)
            (A 0 H 7 | # 1 2 3 4 5 6 # 8 9 => A 0 I 8)
            (A 0 I 8 | # 1 2 3 4 5 6 7 # 9 => A 0 J 9)
            (A 0 J 9 | # 1 2 3 4 5 6 7 8 # => B 1 C 2)
            (B 1 C 2 | 0 # # 3 4 5 6 7 8 9 => B 1 D 3)
            (B 1 D 3 | 0 # 2 # 4 5 6 7 8 9 => B 1 E 4)
            (B 1 E 4 | 0 # 2 3 # 5 6 7 8 9 => B 1 F 5)
            (B 1 F 5 | 0 # 2 3 4 # 6 7 8 9 => B 1 G 6)
            (B 1 G 6 | 0 # 2 3 4 5 # 7 8 9 => B 1 H 7)
            (B 1 H 7 | 0 # 2 3 4 5 6 # 8 9 => B 1 I 8)
            (B 1 I 8 | 0 # 2 3 4 5 6 7 # 9 => B 1 J 9)
            (B 1 J 9 | 0 # 2 3 4 5 6 7 8 # => C 2 D 3)
            (C 2 D 3 | 0 1 # # 4 5 6 7 8 9 => C 2 E 4)
            (C 2 E 4 | 0 1 # 3 # 5 6 7 8 9 => C 2 F 5)
            (C 2 F 5 | 0 1 # 3 4 # 6 7 8 9 => C 2 G 6)
            (C 2 G 6 | 0 1 # 3 4 5 # 7 8 9 => C 2 H 7)
            (C 2 H 7 | 0 1 # 3 4 5 6 # 8 9 => C 2 I 8)
            (C 2 I 8 | 0 1 # 3 4 5 6 7 # 9 => C 2 J 9)
            (C 2 J 9 | 0 1 # 3 4 5 6 7 8 # => D 3 E 4)
            (D 3 E 4 | 0 1 2 # # 5 6 7 8 9 => D 3 F 5)
            (D 3 F 5 | 0 1 2 # 4 # 6 7 8 9 => D 3 G 6)
            (D 3 G 6 | 0 1 2 # 4 5 # 7 8 9 => D 3 H 7)
            (D 3 H 7 | 0 1 2 # 4 5 6 # 8 9 => D 3 I 8)
            (D 3 I 8 | 0 1 2 # 4 5 6 7 # 9 => D 3 J 9)
            (D 3 J 9 | 0 1 2 # 4 5 6 7 8 # => E 4 F 5)
            (E 4 F 5 | 0 1 2 3 # # 6 7 8 9 => E 4 G 6)
            (E 4 G 6 | 0 1 2 3 # 5 # 7 8 9 => E 4 H 7)
            (E 4 H 7 | 0 1 2 3 # 5 6 # 8 9 => E 4 I 8)
            (E 4 I 8 | 0 1 2 3 # 5 6 7 # 9 => E 4 J 9)
            (E 4 J 9 | 0 1 2 3 # 5 6 7 8 # => F 5 G 6)
            (F 5 G 6 | 0 1 2 3 4 # # 7 8 9 => F 5 H 7)
            (F 5 H 7 | 0 1 2 3 4 # 6 # 8 9 => F 5 I 8)
            (F 5 I 8 | 0 1 2 3 4 # 6 7 # 9 => F 5 J 9)
            (F 5 J 9 | 0 1 2 3 4 # 6 7 8 # => G 6 H 7)
            (G 6 H 7 | 0 1 2 3 4 5 # # 8 9 => G 6 I 8)
            (G 6 I 8 | 0 1 2 3 4 5 # 7 # 9 => G 6 J 9)
            (G 6 J 9 | 0 1 2 3 4 5 # 7 8 # => H 7 I 8)
            (H 7 I 8 | 0 1 2 3 4 5 6 # # 9 => H 7 J 9)
            (H 7 J 9 | 0 1 2 3 4 5 6 # 8 # => I 8 J 9)
            (I 8 J 9 | 0 1 2 3 4 5 6 7 # # => @())
    }
    11 {
        @letters: A B C D E F G H I J K
        @shrinking_helper:
            (A 0 B 1 | # # 2 3 4 5 6 7 8 9 10 => A 0 C 2)
            (A 0 C 2 | # 1 # 3 4 5 6 7 8 9 10 => A 0 D 3)
            (A 0 D 3 | # 1 2 # 4 5 6 7 8 9 10 => A 0 E 4)
            (A 0 E 4 | # 1 2 3 # 5 6 7 8 9 10 => A 0 F 5)
            (A 0 F 5 | # 1 2 3 4 # 6 7 8 9 10 => A 0 G 6)
            (A 0 G 6 | # 1 2 3 4 5 # 7 8 9 10 => A 0 H 7)
            (A 0 H 7 | # 1 2 3 4 5 6 # 8 9 10 => A 0 I 8)
            (A 0 I 8 | # 1 2 3 4 5 6 7 # 9 10 => A 0 J 9)
            (A 0 J 9 | # 1 2 3 4 5 6 7 8 # 10 => A 0 K 10)
            (A 0 K 10 | # 1 2 3 4 5 6 7 8 9  # => B 1 C 2)
            (B 1 C 2 | 0 # # 3 4 5 6 7 8 9 10 => B 1 D 3)
            (B 1 D 3 | 0 # 2 # 4 5 6 7 8 9 10 => B 1 E 4)
            (B 1 E 4 | 0 # 2 3 # 5 6 7 8 9 10 => B 1 F 5)
            (B 1 F 5 | 0 # 2 3 4 # 6 7 8 9 10 => B 1 G 6)
            (B 1 G 6 | 0 # 2 3 4 5 # 7 8 9 10 => B 1 H 7)
            (B 1 H 7 | 0 # 2 3 4 5 6 # 8 9 10 => B 1 I 8)
            (B 1 I 8 | 0 # 2 3 4 5 6 7 # 9 10 => B 1 J 9)
            (B 1 J 9 | 0 # 2 3 4 5 6 7 8 # 10 => B 1 K 10)
            (B 1 K 10 | 0 # 2 3 4 5 6 7 8 9  # => C 2 D 3)
            (C 2 D 3 | 0 1 # # 4 5 6 7 8 9 10 => C 2 E 4)
            (C 2 E 4 | 0 1 # 3 # 5 6 7 8 9 10 => C 2 F 5)
            (C 2 F 5 | 0 1 # 3 4 # 6 7 8 9 10 => C 2 G 6)
            (C 2 G 6 | 0 1 # 3 4 5 # 7 8 9 10 => C 2 H 7)
            (C 2 H 7 | 0 1 # 3 4 5 6 # 8 9 10 => C 2 I 8)
            (C 2 I 8 | 0 1 # 3 4 5 6 7 # 9 10 => C 2 J 9)
            (C 2 J 9 | 0 1 # 3 4 5 6 7 8 # 10 => C 2 K 10)
            (C 2 K 10 | 0 1 # 3 4 5 6 7 8 9  # => D 3 E 4)
            (D 3 E 4 | 0 1 2 # # 5 6 7 8 9 10 => D 3 F 5)
            (D 3 F 5 | 0 1 2 # 4 # 6 7 8 9 10 => D 3 G 6)
            (D 3 G 6 | 0 1 2 # 4 5 # 7 8 9 10 => D 3 H 7)
            (D 3 H 7 | 0 1 2 # 4 5 6 # 8 9 10 => D 3 I 8)
            (D 3 I 8 | 0 1 2 # 4 5 6 7 # 9 10 => D 3 J 9)
            (D 3 J 9 | 0 1 2 # 4 5 6 7 8 # 10 => D 3 K 10)
            (D 3 K 10 | 0 1 2 # 4 5 6 7 8 9  # => E 4 F 5)
            (E 4 F 5 | 0 1 2 3 # # 6 7 8 9 10 => E 4 G 6)
            (E 4 G 6 | 0 1 2 3 # 5 # 7 8 9 10 => E 4 H 7)
            (E 4 H 7 | 0 1 2 3 # 5 6 # 8 9 10 => E 4 I 8)
            (E 4 I 8 | 0 1 2 3 # 5 6 7 # 9 10 => E 4 J 9)
            (E 4 J 9 | 0 1 2 3 # 5 6 7 8 # 10 => E 4 K 10)
            (E 4 K 10 | 0 1 2 3 # 5 6 7 8 9  # => F 5 G 6)
            (F 5 G 6 | 0 1 2 3 4 # # 7 8 9 10 => F 5 H 7)
            (F 5 H 7 | 0 1 2 3 4 # 6 # 8 9 10 => F 5 I 8)
            (F 5 I 8 | 0 1 2 3 4 # 6 7 # 9 10 => F 5 J 9)
            (F 5 J 9 | 0 1 2 3 4 # 6 7 8 # 10 => F 5 K 10)
            (F 5 K 10 | 0 1 2 3 4 # 6 7 8 9  # => G 6 H 7)
            (G 6 H 7 | 0 1 2 3 4 5 # # 8 9 10 => G 6 I 8)
            (G 6 I 8 | 0 1 2 3 4 5 # 7 # 9 10 => G 6 J 9)
            (G 6 J 9 | 0 1 2 3 4 5 # 7 8 # 10 => G 6 K 10)
            (G 6 K 10 | 0 1 2 3 4 5 # 7 8 9  # => H 7 I 8)
            (H 7 I 8 | 0 1 2 3 4 5 6 # # 9 10 => H 7 J 9)
            (H 7 J 9 | 0 1 2 3 4 5 6 # 8 # 10 => H 7 K 10)
            (H 7 K 10 | 0 1 2 3 4 5 6 # 8 9  # => I 8 J 9)
            (I 8 J 9 | 0 1 2 3 4 5 6 7 # # 10 => I 8 K 10)
            (I 8 K 10 | 0 1 2 3 4 5 6 7 # 9  # => J 9 K 10)
            (J 9 K 10 | 0 1 2 3 4 5 6 7 8 #  # => @())
    }
    12 {
        @letters: A B C D E F G H I J K L
        @shrinking_helper:
            (A 0 B 1 | # # 2 3 4 5 6 7 8 9 10 11 => A 0 C 2)
            (A 0 C 2 | # 1 # 3 4 5 6 7 8 9 10 11 => A 0 D 3)
            (A 0 D 3 | # 1 2 # 4 5 6 7 8 9 10 11 => A 0 E 4)
            (A 0 E 4 | # 1 2 3 # 5 6 7 8 9 10 11 => A 0 F 5)
            (A 0 F 5 | # 1 2 3 4 # 6 7 8 9 10 11 => A 0 G 6)
            (A 0 G 6 | # 1 2 3 4 5 # 7 8 9 10 11 => A 0 H 7)
            (A 0 H 7 | # 1 2 3 4 5 6 # 8 9 10 11 => A 0 I 8)
            (A 0 I 8 | # 1 2 3 4 5 6 7 # 9 10 11 => A 0 J 9)
            (A 0 J 9 | # 1 2 3 4 5 6 7 8 # 10 11 => A 0 K 10)
            (A 0 K 10 | # 1 2 3 4 5 6 7 8 9  # 11 => A 0 L 11)
            (A 0 L 11 | # 1 2 3 4 5 6 7 8 9 10  # => B 1 C 2)
            (B 1 C 2 | 0 # # 3 4 5 6 7 8 9 10 11 => B 1 D 3)
            (B 1 D 3 | 0 # 2 # 4 5 6 7 8 9 10 11 => B 1 E 4)
            (B 1 E 4 | 0 # 2 3 # 5 6 7 8 9 10 11 => B 1 F 5)
            (B 1 F 5 | 0 # 2 3 4 # 6 7 8 9 10 11 => B 1 G 6)
            (B 1 G 6 | 0 # 2 3 4 5 # 7 8 9 10 11 => B 1 H 7)
            (B 1 H 7 | 0 # 2 3 4 5 6 # 8 9 10 11 => B 1 I 8)
            (B 1 I 8 | 0 # 2 3 4 5 6 7 # 9 10 11 => B 1 J 9)
            (B 1 J 9 | 0 # 2 3 4 5 6 7 8 # 10 11 => B 1 K 10)
            (B 1 K 10 | 0 # 2 3 4 5 6 7 8 9  # 11 => B 1 L 11)
            (B 1 L 11 | 0 # 2 3 4 5 6 7 8 9 10  # => C 2 D 3)
            (C 2 D 3 | 0 1 # # 4 5 6 7 8 9 10 11 => C 2 E 4)
            (C 2 E 4 | 0 1 # 3 # 5 6 7 8 9 10 11 => C 2 F 5)
            (C 2 F 5 | 0 1 # 3 4 # 6 7 8 9 10 11 => C 2 G 6)
            (C 2 G 6 | 0 1 # 3 4 5 # 7 8 9 10 11 => C 2 H 7)
            (C 2 H 7 | 0 1 # 3 4 5 6 # 8 9 10 11 => C 2 I 8)
            (C 2 I 8 | 0 1 # 3 4 5 6 7 # 9 10 11 => C 2 J 9)
            (C 2 J 9 | 0 1 # 3 4 5 6 7 8 # 10 11 => C 2 K 10)
            (C 2 K 10 | 0 1 # 3 4 5 6 7 8 9  # 11 => C 2 L 11)
            (C 2 L 11 | 0 1 # 3 4 5 6 7 8 9 10  # => D 3 E 4)
            (D 3 E 4 | 0 1 2 # # 5 6 7 8 9 10 11 => D 3 F 5)
            (D 3 F 5 | 0 1 2 # 4 # 6 7 8 9 10 11 => D 3 G 6)
            (D 3 G 6 | 0 1 2 # 4 5 # 7 8 9 10 11 => D 3 H 7)
            (D 3 H 7 | 0 1 2 # 4 5 6 # 8 9 10 11 => D 3 I 8)
            (D 3 I 8 | 0 1 2 # 4 5 6 7 # 9 10 11 => D 3 J 9)
            (D 3 J 9 | 0 1 2 # 4 5 6 7 8 # 10 11 => D 3 K 10)
            (D 3 K 10 | 0 1 2 # 4 5 6 7 8 9  # 11 => D 3 L 11)
            (D 3 L 11 | 0 1 2 # 4 5 6 7 8 9 10  # => E 4 F 5)
            (E 4 F 5 | 0 1 2 3 # # 6 7 8 9 10 11 => E 4 G 6)
            (E 4 G 6 | 0 1 2 3 # 5 # 7 8 9 10 11 => E 4 H 7)
            (E 4 H 7 | 0 1 2 3 # 5 6 # 8 9 10 11 => E 4 I 8)
            (E 4 I 8 | 0 1 2 3 # 5 6 7 # 9 10 11 => E 4 J 9)
            (E 4 J 9 | 0 1 2 3 # 5 6 7 8 # 10 11 => E 4 K 10)
            (E 4 K 10 | 0 1 2 3 # 5 6 7 8 9  # 11 => E 4 L 11)
            (E 4 L 11 | 0 1 2 3 # 5 6 7 8 9 10  # => F 5 G 6)
            (F 5 G 6 | 0 1 2 3 4 # # 7 8 9 10 11 => F 5 H 7)
            (F 5 H 7 | 0 1 2 3 4 # 6 # 8 9 10 11 => F 5 I 8)
            (F 5 I 8 | 0 1 2 3 4 # 6 7 # 9 10 11 => F 5 J 9)
            (F 5 J 9 | 0 1 2 3 4 # 6 7 8 # 10 11 => F 5 K 10)
            (F 5 K 10 | 0 1 2 3 4 # 6 7 8 9  # 11 => F 5 L 11)
            (F 5 L 11 | 0 1 2 3 4 # 6 7 8 9 10  # => G 6 H 7)
            (G 6 H 7 | 0 1 2 3 4 5 # # 8 9 10 11 => G 6 I 8)
            (G 6 I 8 | 0 1 2 3 4 5 # 7 # 9 10 11 => G 6 J 9)
            (G 6 J 9 | 0 1 2 3 4 5 # 7 8 # 10 11 => G 6 K 10)
            (G 6 K 10 | 0 1 2 3 4 5 # 7 8 9  # 11 => G 6 L 11)
            (G 6 L 11 | 0 1 2 3 4 5 # 7 8 9 10  # => H 7 I 8)
            (H 7 I 8 | 0 1 2 3 4 5 6 # # 9 10 11 => H 7 J 9)
            (H 7 J 9 | 0 1 2 3 4 5 6 # 8 # 10 11 => H 7 K 10)
            (H 7 K 10 | 0 1 2 3 4 5 6 # 8 9  # 11 => H 7 L 11)
            (H 7 L 11 | 0 1 2 3 4 5 6 # 8 9 10  # => I 8 J 9)
            (I 8 J 9 | 0 1 2 3 4 5 6 7 # # 10 11 => I 8 K 10)
            (I 8 K 10 | 0 1 2 3 4 5 6 7 # 9  # 11 => I 8 L 11)
            (I 8 L 11 | 0 1 2 3 4 5 6 7 # 9 10  # => J 9 K 10)
            (J 9 K 10 | 0 1 2 3 4 5 6 7 8 #  # 11 => J 9 L 11)
            (J 9 L 11 | 0 1 2 3 4 5 6 7 8 # 10  # => K 10 L 11)
            (K 10 L 11 | 0 1 2 3 4 5 6 7 8 9  #  # => @())
    }
}
