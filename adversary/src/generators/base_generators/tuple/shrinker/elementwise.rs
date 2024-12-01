#![expect(
    dead_code,
    reason = "this code is unused until all tuple shrinking is implemented"
)]

use crate::{shrinker::Shrinker, InputGenerator};

macro_rules! elementwise {
    ($(
        $n:literal {
            @ letters : $($letter:ident)+
            @ progress_if_necessary_helper :
                $( ( $shrinking_from:ident $shrinking_to:ident $shrinking_count:literal ) )+
                $shrinking_last:ident
            @ current_attempt_helper : $( (
                $($curr_befores:literal)* $(#)+ $($curr_afters:literal)*
            ) )+
        }
    )*) => {paste::paste!{$(
        // NOTE(ichen): Implementations for tuples of arity 0, 1, and 2 should
        // be manual - this macro's implementation will overcomplicate in
        // meaningful ways.
        const _: () = assert!($n > 2);

        pub struct [<Elementwise $n>]<'gens, $([<Gen $letter>]: InputGenerator),+> {
            current_values: ($([<Gen $letter>]::InputSource),+),
            step: [<Step $n>]<'gens, $([<Gen $letter>]),+>,
        }

        enum [<Step $n>]<'gens, $([<Gen $letter>]: InputGenerator),+> {
            $([<Shrinking $letter>](Box<dyn Shrinker<InputSource = [<Gen $letter>]::InputSource> + 'gens>),)+
            Done,
        }

        impl<'gens, $([<Gen $letter>]: InputGenerator),+>
            [<Elementwise $n>]<'gens, $([<Gen $letter>]),+>
        {
            pub fn new(
                generators: ($(&'gens [<Gen $letter>]),+),
                current_values: ($([<Gen $letter>]::InputSource),+),
            ) -> Self {
                let step = [<Step $n>]::ShrinkingA(Box::new(
                    generators.0.new_shrinker(current_values.0.clone()),
                ));

                let mut this = Self {
                    current_values,
                    step,
                };

                this.progress_if_necessary(generators);

                this
            }

            pub fn progress_if_necessary(&mut self, generators: ($(&'gens [<Gen $letter>]),+)) {
                $(
                    // If the Xth shrinker is done, progress to (X+1)th
                    if let [<Step $n>]::$shrinking_from(shrinker) = &mut self.step {
                        if shrinker.current_attempt().is_none() {
                            self.step = [<Step $n>]::$shrinking_to(Box::new(
                                generators.$shrinking_count.new_shrinker(self.current_values.$shrinking_count.clone()),
                            ));
                        }
                    }
                )+

                // If the last shrinker is done, progress to Done
                if let [<Step $n>]::$shrinking_last(shrinker) = &mut self.step {
                    if shrinker.current_attempt().is_none() {
                        self.step = [<Step $n>]::Done;
                    }
                }
            }

            pub fn is_done(&self) -> bool {
                matches!(self.step, [<Step $n>]::Done)
            }

            pub fn current_attempt(
                &self,
            ) -> Option<($([<Gen $letter>]::InputSource),+)> {
                match &self.step {
                    $(
                        [<Step $n>]::[<Shrinking $letter>](shrinker) => Some((
                            $(self.current_values.$curr_befores.clone(),)*
                            shrinker.current_attempt().unwrap(),
                            $(self.current_values.$curr_afters.clone(),)*
                        )),
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
                    $([<Step $n>]::[<Shrinking $letter>](shrinker) => shrinker.update(current_attempt_passed),)+
                    [<Step $n>]::Done => panic!(concat!("`ElementWise", $n, "::update` called while in `Step::Done`")),
                }

                self.progress_if_necessary(generators);
            }
        }
    )*}};
}

elementwise! {
    3 {
        @letters: A B C
        @progress_if_necessary_helper:
            (ShrinkingA ShrinkingB 1)
            (ShrinkingB ShrinkingC 2)
            ShrinkingC
        @current_attempt_helper:
            (# 1 2)
            (0 # 2)
            (0 1 #)
    }
    4 {
        @letters: A B C D
        @progress_if_necessary_helper:
            (ShrinkingA ShrinkingB 1)
            (ShrinkingB ShrinkingC 2)
            (ShrinkingC ShrinkingD 3)
            ShrinkingD
        @current_attempt_helper:
            (# 1 2 3)
            (0 # 2 3)
            (0 1 # 3)
            (0 1 2 #)
    }
    5 {
        @letters: A B C D E
        @progress_if_necessary_helper:
            (ShrinkingA ShrinkingB 1)
            (ShrinkingB ShrinkingC 2)
            (ShrinkingC ShrinkingD 3)
            (ShrinkingD ShrinkingE 4)
            ShrinkingE
        @current_attempt_helper:
            (# 1 2 3 4)
            (0 # 2 3 4)
            (0 1 # 3 4)
            (0 1 2 # 4)
            (0 1 2 3 #)
    }
    6 {
        @letters: A B C D E F
        @progress_if_necessary_helper:
            (ShrinkingA ShrinkingB 1)
            (ShrinkingB ShrinkingC 2)
            (ShrinkingC ShrinkingD 3)
            (ShrinkingD ShrinkingE 4)
            (ShrinkingE ShrinkingF 5)
            ShrinkingF
        @current_attempt_helper:
            (# 1 2 3 4 5)
            (0 # 2 3 4 5)
            (0 1 # 3 4 5)
            (0 1 2 # 4 5)
            (0 1 2 3 # 5)
            (0 1 2 3 4 #)
    }
    7 {
        @letters: A B C D E F G
        @progress_if_necessary_helper:
            (ShrinkingA ShrinkingB 1)
            (ShrinkingB ShrinkingC 2)
            (ShrinkingC ShrinkingD 3)
            (ShrinkingD ShrinkingE 4)
            (ShrinkingE ShrinkingF 5)
            (ShrinkingF ShrinkingG 6)
            ShrinkingG
        @current_attempt_helper:
            (# 1 2 3 4 5 6)
            (0 # 2 3 4 5 6)
            (0 1 # 3 4 5 6)
            (0 1 2 # 4 5 6)
            (0 1 2 3 # 5 6)
            (0 1 2 3 4 # 6)
            (0 1 2 3 4 5 #)
    }
    8 {
        @letters: A B C D E F G H
        @progress_if_necessary_helper:
            (ShrinkingA ShrinkingB 1)
            (ShrinkingB ShrinkingC 2)
            (ShrinkingC ShrinkingD 3)
            (ShrinkingD ShrinkingE 4)
            (ShrinkingE ShrinkingF 5)
            (ShrinkingF ShrinkingG 6)
            (ShrinkingG ShrinkingH 7)
            ShrinkingH
        @current_attempt_helper:
            (# 1 2 3 4 5 6 7)
            (0 # 2 3 4 5 6 7)
            (0 1 # 3 4 5 6 7)
            (0 1 2 # 4 5 6 7)
            (0 1 2 3 # 5 6 7)
            (0 1 2 3 4 # 6 7)
            (0 1 2 3 4 5 # 7)
            (0 1 2 3 4 5 6 #)
    }
    9 {
        @letters: A B C D E F G H I
        @progress_if_necessary_helper:
            (ShrinkingA ShrinkingB 1)
            (ShrinkingB ShrinkingC 2)
            (ShrinkingC ShrinkingD 3)
            (ShrinkingD ShrinkingE 4)
            (ShrinkingE ShrinkingF 5)
            (ShrinkingF ShrinkingG 6)
            (ShrinkingG ShrinkingH 7)
            (ShrinkingH ShrinkingI 8)
            ShrinkingI
        @current_attempt_helper:
            (# 1 2 3 4 5 6 7 8)
            (0 # 2 3 4 5 6 7 8)
            (0 1 # 3 4 5 6 7 8)
            (0 1 2 # 4 5 6 7 8)
            (0 1 2 3 # 5 6 7 8)
            (0 1 2 3 4 # 6 7 8)
            (0 1 2 3 4 5 # 7 8)
            (0 1 2 3 4 5 6 # 8)
            (0 1 2 3 4 5 6 7 #)
    }
    10 {
        @letters: A B C D E F G H I J
        @progress_if_necessary_helper:
            (ShrinkingA ShrinkingB 1)
            (ShrinkingB ShrinkingC 2)
            (ShrinkingC ShrinkingD 3)
            (ShrinkingD ShrinkingE 4)
            (ShrinkingE ShrinkingF 5)
            (ShrinkingF ShrinkingG 6)
            (ShrinkingG ShrinkingH 7)
            (ShrinkingH ShrinkingI 8)
            (ShrinkingI ShrinkingJ 9)
            ShrinkingJ
        @current_attempt_helper:
            (# 1 2 3 4 5 6 7 8 9)
            (0 # 2 3 4 5 6 7 8 9)
            (0 1 # 3 4 5 6 7 8 9)
            (0 1 2 # 4 5 6 7 8 9)
            (0 1 2 3 # 5 6 7 8 9)
            (0 1 2 3 4 # 6 7 8 9)
            (0 1 2 3 4 5 # 7 8 9)
            (0 1 2 3 4 5 6 # 8 9)
            (0 1 2 3 4 5 6 7 # 9)
            (0 1 2 3 4 5 6 7 8 #)
    }
    11 {
        @letters: A B C D E F G H I J K
        @progress_if_necessary_helper:
            (ShrinkingA ShrinkingB 1)
            (ShrinkingB ShrinkingC 2)
            (ShrinkingC ShrinkingD 3)
            (ShrinkingD ShrinkingE 4)
            (ShrinkingE ShrinkingF 5)
            (ShrinkingF ShrinkingG 6)
            (ShrinkingG ShrinkingH 7)
            (ShrinkingH ShrinkingI 8)
            (ShrinkingI ShrinkingJ 9)
            (ShrinkingJ ShrinkingK 10)
            ShrinkingK
        @current_attempt_helper:
            (# 1 2 3 4 5 6 7 8 9 10)
            (0 # 2 3 4 5 6 7 8 9 10)
            (0 1 # 3 4 5 6 7 8 9 10)
            (0 1 2 # 4 5 6 7 8 9 10)
            (0 1 2 3 # 5 6 7 8 9 10)
            (0 1 2 3 4 # 6 7 8 9 10)
            (0 1 2 3 4 5 # 7 8 9 10)
            (0 1 2 3 4 5 6 # 8 9 10)
            (0 1 2 3 4 5 6 7 # 9 10)
            (0 1 2 3 4 5 6 7 8 # 10)
            (0 1 2 3 4 5 6 7 8 9 ##)
    }
    12 {
        @letters: A B C D E F G H I J K L
        @progress_if_necessary_helper:
            (ShrinkingA ShrinkingB 1)
            (ShrinkingB ShrinkingC 2)
            (ShrinkingC ShrinkingD 3)
            (ShrinkingD ShrinkingE 4)
            (ShrinkingE ShrinkingF 5)
            (ShrinkingF ShrinkingG 6)
            (ShrinkingG ShrinkingH 7)
            (ShrinkingH ShrinkingI 8)
            (ShrinkingI ShrinkingJ 9)
            (ShrinkingJ ShrinkingK 10)
            (ShrinkingK ShrinkingL 11)
            ShrinkingL
        @current_attempt_helper:
            (# 1 2 3 4 5 6 7 8 9 10 11)
            (0 # 2 3 4 5 6 7 8 9 10 11)
            (0 1 # 3 4 5 6 7 8 9 10 11)
            (0 1 2 # 4 5 6 7 8 9 10 11)
            (0 1 2 3 # 5 6 7 8 9 10 11)
            (0 1 2 3 4 # 6 7 8 9 10 11)
            (0 1 2 3 4 5 # 7 8 9 10 11)
            (0 1 2 3 4 5 6 # 8 9 10 11)
            (0 1 2 3 4 5 6 7 # 9 10 11)
            (0 1 2 3 4 5 6 7 8 # 10 11)
            (0 1 2 3 4 5 6 7 8 9 ## 11)
            (0 1 2 3 4 5 6 7 8 9 10 ##)
    }
}
