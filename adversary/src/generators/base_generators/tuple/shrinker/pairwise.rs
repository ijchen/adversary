use crate::{ValueGen, shrinker::Shrinker as _};

use super::super::Pair;

macro_rules! pairwise {
    ($(
        $n:literal {
            @ letters : $($letter:ident)+
            @ shrinking_helper :
                $( (
                    $shrinking_first:ident
                    $shrinking_second:ident
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

        pub struct [<Pairwise $n>]<'gens, $([<Gen $letter>]: ValueGen),+> {
            current_values: ($([<Gen $letter>]::Seed),+),
            step: [<Step $n>]<'gens, $([<Gen $letter>]),+>,
        }

        // TODO(ichen): is there a nicer way to do this than an enum for every possible
        // combination of two generators?
        enum [<Step $n>]<'gens, $([<Gen $letter>]: ValueGen),+> {
            $([<Shrinking $shrinking_first $shrinking_second>](
                Pair<'gens, [<Gen $shrinking_first>], [<Gen $shrinking_second>]>
            ),)+
            Done,
        }

        impl<'gens, $([<Gen $letter>]: ValueGen),+>
            [<Pairwise $n>]<'gens, $([<Gen $letter>]),+>
        {
            pub fn new(
                generators: ($(&'gens [<Gen $letter>]),+),
                current_values: ($([<Gen $letter>]::Seed),+),
            ) -> Self {
                let step = [<Step $n>]::ShrinkingAB(Pair::new(
                    generators.0.new_shrinker(current_values.0.clone()),
                    generators.1.new_shrinker(current_values.1.clone()),
                    current_values.0.clone(),
                    current_values.1.clone(),
                ));

                let mut this = Self {
                    current_values,
                    step,
                };

                this.progress_if_necessary(generators);

                this
            }

            pub fn progress_if_necessary(&mut self, generators: ($(&'gens [<Gen $letter>]),+)) {
                // Probably the most sauced macro by example I've ever written
                $(
                    $(
                        // If a pair is done, progress to the next pair
                        if let [<Step $n>]::[<Shrinking $shrinking_first $shrinking_second>](pair) = &mut self.step {
                            if pair.is_done() {
                                self.step = [<Step $n>]::[<Shrinking $shrinking_to_first_letter $shrinking_to_second_letter>](Pair::new(
                                    generators.$shrinking_to_first_index.new_shrinker(self.current_values.$shrinking_to_first_index.clone()),
                                    generators.$shrinking_to_second_index.new_shrinker(self.current_values.$shrinking_to_second_index.clone()),
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
                        if let [<Step $n>]::[<Shrinking $shrinking_first $shrinking_second>](pair) = &mut self.step {
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
                        [<Step $n>]::[<Shrinking $shrinking_first $shrinking_second>](pair) => {
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
                generators: ($(&'gens [<Gen $letter>]),+),
                current_attempt_passed: bool,
            ) {
                match &mut self.step {
                    $([<Step $n>]::[<Shrinking $shrinking_first $shrinking_second>](pair) => pair.update(current_attempt_passed),)+
                    [<Step $n>]::Done => panic!(concat!("`Elementwise", $n, "::update` called while in `Step::Done`")),
                }

                self.progress_if_necessary(generators);
            }
        }
    )*}};
}

// Macro invocation code generated with:
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=0611ce326ddeb0c4a53c7f5638913aa8
pairwise! {
    3 {
        @letters: A B C
        @shrinking_helper:
            (A B | # # 2 => A 0 C 2)
            (A C | # 1 # => B 1 C 2)
            (B C | 0 # # => @())
    }
    4 {
        @letters: A B C D
        @shrinking_helper:
            (A B | # # 2 3 => A 0 C 2)
            (A C | # 1 # 3 => A 0 D 3)
            (A D | # 1 2 # => B 1 C 2)
            (B C | 0 # # 3 => B 1 D 3)
            (B D | 0 # 2 # => C 2 D 3)
            (C D | 0 1 # # => @())
    }
    5 {
        @letters: A B C D E
        @shrinking_helper:
            (A B | # # 2 3 4 => A 0 C 2)
            (A C | # 1 # 3 4 => A 0 D 3)
            (A D | # 1 2 # 4 => A 0 E 4)
            (A E | # 1 2 3 # => B 1 C 2)
            (B C | 0 # # 3 4 => B 1 D 3)
            (B D | 0 # 2 # 4 => B 1 E 4)
            (B E | 0 # 2 3 # => C 2 D 3)
            (C D | 0 1 # # 4 => C 2 E 4)
            (C E | 0 1 # 3 # => D 3 E 4)
            (D E | 0 1 2 # # => @())
    }
    6 {
        @letters: A B C D E F
        @shrinking_helper:
            (A B | # # 2 3 4 5 => A 0 C 2)
            (A C | # 1 # 3 4 5 => A 0 D 3)
            (A D | # 1 2 # 4 5 => A 0 E 4)
            (A E | # 1 2 3 # 5 => A 0 F 5)
            (A F | # 1 2 3 4 # => B 1 C 2)
            (B C | 0 # # 3 4 5 => B 1 D 3)
            (B D | 0 # 2 # 4 5 => B 1 E 4)
            (B E | 0 # 2 3 # 5 => B 1 F 5)
            (B F | 0 # 2 3 4 # => C 2 D 3)
            (C D | 0 1 # # 4 5 => C 2 E 4)
            (C E | 0 1 # 3 # 5 => C 2 F 5)
            (C F | 0 1 # 3 4 # => D 3 E 4)
            (D E | 0 1 2 # # 5 => D 3 F 5)
            (D F | 0 1 2 # 4 # => E 4 F 5)
            (E F | 0 1 2 3 # # => @())
    }
    7 {
        @letters: A B C D E F G
        @shrinking_helper:
            (A B | # # 2 3 4 5 6 => A 0 C 2)
            (A C | # 1 # 3 4 5 6 => A 0 D 3)
            (A D | # 1 2 # 4 5 6 => A 0 E 4)
            (A E | # 1 2 3 # 5 6 => A 0 F 5)
            (A F | # 1 2 3 4 # 6 => A 0 G 6)
            (A G | # 1 2 3 4 5 # => B 1 C 2)
            (B C | 0 # # 3 4 5 6 => B 1 D 3)
            (B D | 0 # 2 # 4 5 6 => B 1 E 4)
            (B E | 0 # 2 3 # 5 6 => B 1 F 5)
            (B F | 0 # 2 3 4 # 6 => B 1 G 6)
            (B G | 0 # 2 3 4 5 # => C 2 D 3)
            (C D | 0 1 # # 4 5 6 => C 2 E 4)
            (C E | 0 1 # 3 # 5 6 => C 2 F 5)
            (C F | 0 1 # 3 4 # 6 => C 2 G 6)
            (C G | 0 1 # 3 4 5 # => D 3 E 4)
            (D E | 0 1 2 # # 5 6 => D 3 F 5)
            (D F | 0 1 2 # 4 # 6 => D 3 G 6)
            (D G | 0 1 2 # 4 5 # => E 4 F 5)
            (E F | 0 1 2 3 # # 6 => E 4 G 6)
            (E G | 0 1 2 3 # 5 # => F 5 G 6)
            (F G | 0 1 2 3 4 # # => @())
    }
    8 {
        @letters: A B C D E F G H
        @shrinking_helper:
            (A B | # # 2 3 4 5 6 7 => A 0 C 2)
            (A C | # 1 # 3 4 5 6 7 => A 0 D 3)
            (A D | # 1 2 # 4 5 6 7 => A 0 E 4)
            (A E | # 1 2 3 # 5 6 7 => A 0 F 5)
            (A F | # 1 2 3 4 # 6 7 => A 0 G 6)
            (A G | # 1 2 3 4 5 # 7 => A 0 H 7)
            (A H | # 1 2 3 4 5 6 # => B 1 C 2)
            (B C | 0 # # 3 4 5 6 7 => B 1 D 3)
            (B D | 0 # 2 # 4 5 6 7 => B 1 E 4)
            (B E | 0 # 2 3 # 5 6 7 => B 1 F 5)
            (B F | 0 # 2 3 4 # 6 7 => B 1 G 6)
            (B G | 0 # 2 3 4 5 # 7 => B 1 H 7)
            (B H | 0 # 2 3 4 5 6 # => C 2 D 3)
            (C D | 0 1 # # 4 5 6 7 => C 2 E 4)
            (C E | 0 1 # 3 # 5 6 7 => C 2 F 5)
            (C F | 0 1 # 3 4 # 6 7 => C 2 G 6)
            (C G | 0 1 # 3 4 5 # 7 => C 2 H 7)
            (C H | 0 1 # 3 4 5 6 # => D 3 E 4)
            (D E | 0 1 2 # # 5 6 7 => D 3 F 5)
            (D F | 0 1 2 # 4 # 6 7 => D 3 G 6)
            (D G | 0 1 2 # 4 5 # 7 => D 3 H 7)
            (D H | 0 1 2 # 4 5 6 # => E 4 F 5)
            (E F | 0 1 2 3 # # 6 7 => E 4 G 6)
            (E G | 0 1 2 3 # 5 # 7 => E 4 H 7)
            (E H | 0 1 2 3 # 5 6 # => F 5 G 6)
            (F G | 0 1 2 3 4 # # 7 => F 5 H 7)
            (F H | 0 1 2 3 4 # 6 # => G 6 H 7)
            (G H | 0 1 2 3 4 5 # # => @())
    }
    9 {
        @letters: A B C D E F G H I
        @shrinking_helper:
            (A B | # # 2 3 4 5 6 7 8 => A 0 C 2)
            (A C | # 1 # 3 4 5 6 7 8 => A 0 D 3)
            (A D | # 1 2 # 4 5 6 7 8 => A 0 E 4)
            (A E | # 1 2 3 # 5 6 7 8 => A 0 F 5)
            (A F | # 1 2 3 4 # 6 7 8 => A 0 G 6)
            (A G | # 1 2 3 4 5 # 7 8 => A 0 H 7)
            (A H | # 1 2 3 4 5 6 # 8 => A 0 I 8)
            (A I | # 1 2 3 4 5 6 7 # => B 1 C 2)
            (B C | 0 # # 3 4 5 6 7 8 => B 1 D 3)
            (B D | 0 # 2 # 4 5 6 7 8 => B 1 E 4)
            (B E | 0 # 2 3 # 5 6 7 8 => B 1 F 5)
            (B F | 0 # 2 3 4 # 6 7 8 => B 1 G 6)
            (B G | 0 # 2 3 4 5 # 7 8 => B 1 H 7)
            (B H | 0 # 2 3 4 5 6 # 8 => B 1 I 8)
            (B I | 0 # 2 3 4 5 6 7 # => C 2 D 3)
            (C D | 0 1 # # 4 5 6 7 8 => C 2 E 4)
            (C E | 0 1 # 3 # 5 6 7 8 => C 2 F 5)
            (C F | 0 1 # 3 4 # 6 7 8 => C 2 G 6)
            (C G | 0 1 # 3 4 5 # 7 8 => C 2 H 7)
            (C H | 0 1 # 3 4 5 6 # 8 => C 2 I 8)
            (C I | 0 1 # 3 4 5 6 7 # => D 3 E 4)
            (D E | 0 1 2 # # 5 6 7 8 => D 3 F 5)
            (D F | 0 1 2 # 4 # 6 7 8 => D 3 G 6)
            (D G | 0 1 2 # 4 5 # 7 8 => D 3 H 7)
            (D H | 0 1 2 # 4 5 6 # 8 => D 3 I 8)
            (D I | 0 1 2 # 4 5 6 7 # => E 4 F 5)
            (E F | 0 1 2 3 # # 6 7 8 => E 4 G 6)
            (E G | 0 1 2 3 # 5 # 7 8 => E 4 H 7)
            (E H | 0 1 2 3 # 5 6 # 8 => E 4 I 8)
            (E I | 0 1 2 3 # 5 6 7 # => F 5 G 6)
            (F G | 0 1 2 3 4 # # 7 8 => F 5 H 7)
            (F H | 0 1 2 3 4 # 6 # 8 => F 5 I 8)
            (F I | 0 1 2 3 4 # 6 7 # => G 6 H 7)
            (G H | 0 1 2 3 4 5 # # 8 => G 6 I 8)
            (G I | 0 1 2 3 4 5 # 7 # => H 7 I 8)
            (H I | 0 1 2 3 4 5 6 # # => @())
    }
    10 {
        @letters: A B C D E F G H I J
        @shrinking_helper:
            (A B | # # 2 3 4 5 6 7 8 9 => A 0 C 2)
            (A C | # 1 # 3 4 5 6 7 8 9 => A 0 D 3)
            (A D | # 1 2 # 4 5 6 7 8 9 => A 0 E 4)
            (A E | # 1 2 3 # 5 6 7 8 9 => A 0 F 5)
            (A F | # 1 2 3 4 # 6 7 8 9 => A 0 G 6)
            (A G | # 1 2 3 4 5 # 7 8 9 => A 0 H 7)
            (A H | # 1 2 3 4 5 6 # 8 9 => A 0 I 8)
            (A I | # 1 2 3 4 5 6 7 # 9 => A 0 J 9)
            (A J | # 1 2 3 4 5 6 7 8 # => B 1 C 2)
            (B C | 0 # # 3 4 5 6 7 8 9 => B 1 D 3)
            (B D | 0 # 2 # 4 5 6 7 8 9 => B 1 E 4)
            (B E | 0 # 2 3 # 5 6 7 8 9 => B 1 F 5)
            (B F | 0 # 2 3 4 # 6 7 8 9 => B 1 G 6)
            (B G | 0 # 2 3 4 5 # 7 8 9 => B 1 H 7)
            (B H | 0 # 2 3 4 5 6 # 8 9 => B 1 I 8)
            (B I | 0 # 2 3 4 5 6 7 # 9 => B 1 J 9)
            (B J | 0 # 2 3 4 5 6 7 8 # => C 2 D 3)
            (C D | 0 1 # # 4 5 6 7 8 9 => C 2 E 4)
            (C E | 0 1 # 3 # 5 6 7 8 9 => C 2 F 5)
            (C F | 0 1 # 3 4 # 6 7 8 9 => C 2 G 6)
            (C G | 0 1 # 3 4 5 # 7 8 9 => C 2 H 7)
            (C H | 0 1 # 3 4 5 6 # 8 9 => C 2 I 8)
            (C I | 0 1 # 3 4 5 6 7 # 9 => C 2 J 9)
            (C J | 0 1 # 3 4 5 6 7 8 # => D 3 E 4)
            (D E | 0 1 2 # # 5 6 7 8 9 => D 3 F 5)
            (D F | 0 1 2 # 4 # 6 7 8 9 => D 3 G 6)
            (D G | 0 1 2 # 4 5 # 7 8 9 => D 3 H 7)
            (D H | 0 1 2 # 4 5 6 # 8 9 => D 3 I 8)
            (D I | 0 1 2 # 4 5 6 7 # 9 => D 3 J 9)
            (D J | 0 1 2 # 4 5 6 7 8 # => E 4 F 5)
            (E F | 0 1 2 3 # # 6 7 8 9 => E 4 G 6)
            (E G | 0 1 2 3 # 5 # 7 8 9 => E 4 H 7)
            (E H | 0 1 2 3 # 5 6 # 8 9 => E 4 I 8)
            (E I | 0 1 2 3 # 5 6 7 # 9 => E 4 J 9)
            (E J | 0 1 2 3 # 5 6 7 8 # => F 5 G 6)
            (F G | 0 1 2 3 4 # # 7 8 9 => F 5 H 7)
            (F H | 0 1 2 3 4 # 6 # 8 9 => F 5 I 8)
            (F I | 0 1 2 3 4 # 6 7 # 9 => F 5 J 9)
            (F J | 0 1 2 3 4 # 6 7 8 # => G 6 H 7)
            (G H | 0 1 2 3 4 5 # # 8 9 => G 6 I 8)
            (G I | 0 1 2 3 4 5 # 7 # 9 => G 6 J 9)
            (G J | 0 1 2 3 4 5 # 7 8 # => H 7 I 8)
            (H I | 0 1 2 3 4 5 6 # # 9 => H 7 J 9)
            (H J | 0 1 2 3 4 5 6 # 8 # => I 8 J 9)
            (I J | 0 1 2 3 4 5 6 7 # # => @())
    }
    11 {
        @letters: A B C D E F G H I J K
        @shrinking_helper:
            (A B | # # 2 3 4 5 6 7 8 9 10 => A 0 C 2)
            (A C | # 1 # 3 4 5 6 7 8 9 10 => A 0 D 3)
            (A D | # 1 2 # 4 5 6 7 8 9 10 => A 0 E 4)
            (A E | # 1 2 3 # 5 6 7 8 9 10 => A 0 F 5)
            (A F | # 1 2 3 4 # 6 7 8 9 10 => A 0 G 6)
            (A G | # 1 2 3 4 5 # 7 8 9 10 => A 0 H 7)
            (A H | # 1 2 3 4 5 6 # 8 9 10 => A 0 I 8)
            (A I | # 1 2 3 4 5 6 7 # 9 10 => A 0 J 9)
            (A J | # 1 2 3 4 5 6 7 8 # 10 => A 0 K 10)
            (A K | # 1 2 3 4 5 6 7 8 9  # => B 1 C 2)
            (B C | 0 # # 3 4 5 6 7 8 9 10 => B 1 D 3)
            (B D | 0 # 2 # 4 5 6 7 8 9 10 => B 1 E 4)
            (B E | 0 # 2 3 # 5 6 7 8 9 10 => B 1 F 5)
            (B F | 0 # 2 3 4 # 6 7 8 9 10 => B 1 G 6)
            (B G | 0 # 2 3 4 5 # 7 8 9 10 => B 1 H 7)
            (B H | 0 # 2 3 4 5 6 # 8 9 10 => B 1 I 8)
            (B I | 0 # 2 3 4 5 6 7 # 9 10 => B 1 J 9)
            (B J | 0 # 2 3 4 5 6 7 8 # 10 => B 1 K 10)
            (B K | 0 # 2 3 4 5 6 7 8 9  # => C 2 D 3)
            (C D | 0 1 # # 4 5 6 7 8 9 10 => C 2 E 4)
            (C E | 0 1 # 3 # 5 6 7 8 9 10 => C 2 F 5)
            (C F | 0 1 # 3 4 # 6 7 8 9 10 => C 2 G 6)
            (C G | 0 1 # 3 4 5 # 7 8 9 10 => C 2 H 7)
            (C H | 0 1 # 3 4 5 6 # 8 9 10 => C 2 I 8)
            (C I | 0 1 # 3 4 5 6 7 # 9 10 => C 2 J 9)
            (C J | 0 1 # 3 4 5 6 7 8 # 10 => C 2 K 10)
            (C K | 0 1 # 3 4 5 6 7 8 9  # => D 3 E 4)
            (D E | 0 1 2 # # 5 6 7 8 9 10 => D 3 F 5)
            (D F | 0 1 2 # 4 # 6 7 8 9 10 => D 3 G 6)
            (D G | 0 1 2 # 4 5 # 7 8 9 10 => D 3 H 7)
            (D H | 0 1 2 # 4 5 6 # 8 9 10 => D 3 I 8)
            (D I | 0 1 2 # 4 5 6 7 # 9 10 => D 3 J 9)
            (D J | 0 1 2 # 4 5 6 7 8 # 10 => D 3 K 10)
            (D K | 0 1 2 # 4 5 6 7 8 9  # => E 4 F 5)
            (E F | 0 1 2 3 # # 6 7 8 9 10 => E 4 G 6)
            (E G | 0 1 2 3 # 5 # 7 8 9 10 => E 4 H 7)
            (E H | 0 1 2 3 # 5 6 # 8 9 10 => E 4 I 8)
            (E I | 0 1 2 3 # 5 6 7 # 9 10 => E 4 J 9)
            (E J | 0 1 2 3 # 5 6 7 8 # 10 => E 4 K 10)
            (E K | 0 1 2 3 # 5 6 7 8 9  # => F 5 G 6)
            (F G | 0 1 2 3 4 # # 7 8 9 10 => F 5 H 7)
            (F H | 0 1 2 3 4 # 6 # 8 9 10 => F 5 I 8)
            (F I | 0 1 2 3 4 # 6 7 # 9 10 => F 5 J 9)
            (F J | 0 1 2 3 4 # 6 7 8 # 10 => F 5 K 10)
            (F K | 0 1 2 3 4 # 6 7 8 9  # => G 6 H 7)
            (G H | 0 1 2 3 4 5 # # 8 9 10 => G 6 I 8)
            (G I | 0 1 2 3 4 5 # 7 # 9 10 => G 6 J 9)
            (G J | 0 1 2 3 4 5 # 7 8 # 10 => G 6 K 10)
            (G K | 0 1 2 3 4 5 # 7 8 9  # => H 7 I 8)
            (H I | 0 1 2 3 4 5 6 # # 9 10 => H 7 J 9)
            (H J | 0 1 2 3 4 5 6 # 8 # 10 => H 7 K 10)
            (H K | 0 1 2 3 4 5 6 # 8 9  # => I 8 J 9)
            (I J | 0 1 2 3 4 5 6 7 # # 10 => I 8 K 10)
            (I K | 0 1 2 3 4 5 6 7 # 9  # => J 9 K 10)
            (J K | 0 1 2 3 4 5 6 7 8 #  # => @())
    }
    12 {
        @letters: A B C D E F G H I J K L
        @shrinking_helper:
            (A B | # # 2 3 4 5 6 7 8 9 10 11 => A 0 C 2)
            (A C | # 1 # 3 4 5 6 7 8 9 10 11 => A 0 D 3)
            (A D | # 1 2 # 4 5 6 7 8 9 10 11 => A 0 E 4)
            (A E | # 1 2 3 # 5 6 7 8 9 10 11 => A 0 F 5)
            (A F | # 1 2 3 4 # 6 7 8 9 10 11 => A 0 G 6)
            (A G | # 1 2 3 4 5 # 7 8 9 10 11 => A 0 H 7)
            (A H | # 1 2 3 4 5 6 # 8 9 10 11 => A 0 I 8)
            (A I | # 1 2 3 4 5 6 7 # 9 10 11 => A 0 J 9)
            (A J | # 1 2 3 4 5 6 7 8 # 10 11 => A 0 K 10)
            (A K | # 1 2 3 4 5 6 7 8 9  # 11 => A 0 L 11)
            (A L | # 1 2 3 4 5 6 7 8 9 10  # => B 1 C 2)
            (B C | 0 # # 3 4 5 6 7 8 9 10 11 => B 1 D 3)
            (B D | 0 # 2 # 4 5 6 7 8 9 10 11 => B 1 E 4)
            (B E | 0 # 2 3 # 5 6 7 8 9 10 11 => B 1 F 5)
            (B F | 0 # 2 3 4 # 6 7 8 9 10 11 => B 1 G 6)
            (B G | 0 # 2 3 4 5 # 7 8 9 10 11 => B 1 H 7)
            (B H | 0 # 2 3 4 5 6 # 8 9 10 11 => B 1 I 8)
            (B I | 0 # 2 3 4 5 6 7 # 9 10 11 => B 1 J 9)
            (B J | 0 # 2 3 4 5 6 7 8 # 10 11 => B 1 K 10)
            (B K | 0 # 2 3 4 5 6 7 8 9  # 11 => B 1 L 11)
            (B L | 0 # 2 3 4 5 6 7 8 9 10  # => C 2 D 3)
            (C D | 0 1 # # 4 5 6 7 8 9 10 11 => C 2 E 4)
            (C E | 0 1 # 3 # 5 6 7 8 9 10 11 => C 2 F 5)
            (C F | 0 1 # 3 4 # 6 7 8 9 10 11 => C 2 G 6)
            (C G | 0 1 # 3 4 5 # 7 8 9 10 11 => C 2 H 7)
            (C H | 0 1 # 3 4 5 6 # 8 9 10 11 => C 2 I 8)
            (C I | 0 1 # 3 4 5 6 7 # 9 10 11 => C 2 J 9)
            (C J | 0 1 # 3 4 5 6 7 8 # 10 11 => C 2 K 10)
            (C K | 0 1 # 3 4 5 6 7 8 9  # 11 => C 2 L 11)
            (C L | 0 1 # 3 4 5 6 7 8 9 10  # => D 3 E 4)
            (D E | 0 1 2 # # 5 6 7 8 9 10 11 => D 3 F 5)
            (D F | 0 1 2 # 4 # 6 7 8 9 10 11 => D 3 G 6)
            (D G | 0 1 2 # 4 5 # 7 8 9 10 11 => D 3 H 7)
            (D H | 0 1 2 # 4 5 6 # 8 9 10 11 => D 3 I 8)
            (D I | 0 1 2 # 4 5 6 7 # 9 10 11 => D 3 J 9)
            (D J | 0 1 2 # 4 5 6 7 8 # 10 11 => D 3 K 10)
            (D K | 0 1 2 # 4 5 6 7 8 9  # 11 => D 3 L 11)
            (D L | 0 1 2 # 4 5 6 7 8 9 10  # => E 4 F 5)
            (E F | 0 1 2 3 # # 6 7 8 9 10 11 => E 4 G 6)
            (E G | 0 1 2 3 # 5 # 7 8 9 10 11 => E 4 H 7)
            (E H | 0 1 2 3 # 5 6 # 8 9 10 11 => E 4 I 8)
            (E I | 0 1 2 3 # 5 6 7 # 9 10 11 => E 4 J 9)
            (E J | 0 1 2 3 # 5 6 7 8 # 10 11 => E 4 K 10)
            (E K | 0 1 2 3 # 5 6 7 8 9  # 11 => E 4 L 11)
            (E L | 0 1 2 3 # 5 6 7 8 9 10  # => F 5 G 6)
            (F G | 0 1 2 3 4 # # 7 8 9 10 11 => F 5 H 7)
            (F H | 0 1 2 3 4 # 6 # 8 9 10 11 => F 5 I 8)
            (F I | 0 1 2 3 4 # 6 7 # 9 10 11 => F 5 J 9)
            (F J | 0 1 2 3 4 # 6 7 8 # 10 11 => F 5 K 10)
            (F K | 0 1 2 3 4 # 6 7 8 9  # 11 => F 5 L 11)
            (F L | 0 1 2 3 4 # 6 7 8 9 10  # => G 6 H 7)
            (G H | 0 1 2 3 4 5 # # 8 9 10 11 => G 6 I 8)
            (G I | 0 1 2 3 4 5 # 7 # 9 10 11 => G 6 J 9)
            (G J | 0 1 2 3 4 5 # 7 8 # 10 11 => G 6 K 10)
            (G K | 0 1 2 3 4 5 # 7 8 9  # 11 => G 6 L 11)
            (G L | 0 1 2 3 4 5 # 7 8 9 10  # => H 7 I 8)
            (H I | 0 1 2 3 4 5 6 # # 9 10 11 => H 7 J 9)
            (H J | 0 1 2 3 4 5 6 # 8 # 10 11 => H 7 K 10)
            (H K | 0 1 2 3 4 5 6 # 8 9  # 11 => H 7 L 11)
            (H L | 0 1 2 3 4 5 6 # 8 9 10  # => I 8 J 9)
            (I J | 0 1 2 3 4 5 6 7 # # 10 11 => I 8 K 10)
            (I K | 0 1 2 3 4 5 6 7 # 9  # 11 => I 8 L 11)
            (I L | 0 1 2 3 4 5 6 7 # 9 10  # => J 9 K 10)
            (J K | 0 1 2 3 4 5 6 7 8 #  # 11 => J 9 L 11)
            (J L | 0 1 2 3 4 5 6 7 8 # 10  # => K 10 L 11)
            (K L | 0 1 2 3 4 5 6 7 8 9  #  # => @())
    }
}
