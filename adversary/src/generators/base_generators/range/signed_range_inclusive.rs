// use std::{collections::HashSet, ops::RangeInclusive};

// use crate::{input_generator::NextAttempt, InputGenerator};

// // TODO(ichen): I'd really like this struct to be private - I don't want to make
// // any API promises about it.
// pub struct SignedRangeHistory<T> {
//     min_abs_failing: T,
//     max_abs_passing: Option<T>,
// }

// macro_rules! signed_range_inclusive {
//     ($($t: ty),+$(,)?) => {
//         $(
//             impl InputGenerator for RangeInclusive<$t> {
//                 type Input = $t;

//                 type InputSource = Self::Input;

//                 type History = SignedRangeHistory<$t>;

//                 fn cardinality(&self) -> Option<usize> {
//                     assert!(!self.is_empty());

//                     // TODO(ichen): I haven't fully thought through if overflow
//                     // can cause issues here (especially when coming from other
//                     // Range types)
//                     usize::try_from(self.end().abs_diff(*self.start()))
//                         .ok()
//                         .and_then(|n| n.checked_add(1))
//                 }

//                 fn exhaustive(
//                     &self,
//                 ) -> impl Iterator<Item = Self::InputSource> {
//                     assert!(!self.is_empty());

//                     self.clone()
//                 }

//                 fn adversarial_count(&self) -> Option<usize> {
//                     assert!(!self.is_empty());

//                     // TODO(ichen): will eventually want to implement this
//                     // without actually calling .adversarial()
//                     Some(self.adversarial().count())
//                 }

//                 fn adversarial(
//                     &self,
//                 ) -> impl Iterator<Item = Self::InputSource> {
//                     // TODO(ichen): see if we can make this const evaluatable
//                     // (assuming the compiler knows the range bounds at compile
//                     // time). If not, at least optimize it to be as fast as we
//                     // can get it... this HashSet stuff is almost certainly
//                     // going to be very slow

//                     assert!(!self.is_empty());

//                     HashSet::from([*self.start(), self.start() + 1, -1, 0, 1, self.end() - 1, *self.end()])
//                         .into_iter()
//                         .filter(|n| self.contains(n))
//                 }

//                 fn sample(
//                     &self,
//                     rng: &mut (impl rand::Rng + ?Sized),
//                 ) -> Self::InputSource {
//                     assert!(!self.is_empty());

//                     rng.gen_range(self.clone())
//                 }

//                 fn new_history(&self, failing_input: Self::InputSource) -> Self::History {
//                     assert!(!self.is_empty());

//                     SignedRangeHistory {
//                         min_abs_failing: failing_input,
//                         max_abs_passing: None,
//                     }
//                 }

//                 fn current_simplest_failing(&self, history: &Self::History) -> Self::InputSource {
//                     history.min_abs_failing
//                 }

//                 fn next_input(
//                     &self,
//                     _rng: &mut impl rand::Rng,
//                     history: &Self::History,
//                 ) -> NextAttempt<Self::InputSource> {
//                     assert!(!self.is_empty());

//                     let min_abs_possible = if self.contains(&0) {
//                         0
//                     } else if self.start().unsigned_abs() < self.end().unsigned_abs() {
//                         *self.start()
//                     } else {
//                         *self.end()
//                     };

//                     // If we already know the minimum (abs) value is failing,
//                     // we're done shrinking
//                     if history.min_abs_failing == min_abs_possible {
//                         return NextAttempt::Done;
//                     }

//                     // If we don't have a lower bound, try the minimum value
//                     if history.max_abs_passing.is_none() {
//                         return NextAttempt::ShrinkAttempt(min_abs_possible);
//                     }

//                     // We have a minimum and maximum
//                     let min_failing = history.min_abs_failing;
//                     let max_passing = history.max_abs_passing.unwrap();

//                     // TODO: document as an invariant
//                     assert!(max_passing.unsigned_abs() < min_failing.unsigned_abs());

//                     // If there's nothing between the upper and lower bounds,
//                     // we're done searching.
//                     if max_passing.unsigned_abs() + 1 == min_failing.unsigned_abs() {
//                         return NextAttempt::Done;
//                     }

//                     // If we're not done, try the midpoint of our upper and
//                     // lower bounds
//                     let next_attempt = (min_failing - max_passing) / 2 + max_passing;
//                     NextAttempt::ShrinkAttempt(next_attempt)
//                 }

//                 fn update_history(
//                     &self,
//                     history: &mut Self::History,
//                     shrinkable_input: Self::InputSource,
//                     test_passed: bool,
//                 ) {
//                     assert!(!self.is_empty());

//                     // If the test passed, update the lower bound
//                     if test_passed {
//                         assert!(history
//                             .max_abs_passing
//                             .map_or(true, |max_passing| shrinkable_input.unsigned_abs() > max_passing.unsigned_abs()));

//                         history.max_abs_passing = Some(shrinkable_input);
//                     }
//                     // If the test failed, update the upper bound
//                     else {
//                         assert!(shrinkable_input.unsigned_abs() < history.min_abs_failing.unsigned_abs());

//                         history.min_abs_failing = shrinkable_input;
//                     }
//                 }

//                 fn generate_observations(&self, _history: Self::History) -> Vec<Observation> {
//                     assert!(!self.is_empty());

//                     // TODO: be helpful
//                     vec![]
//                 }

//                 fn create_input(&self, input_source: Self::InputSource) -> Self::Input {
//                     input_source
//                 }
//             }
//         )+
//     };
// }

// signed_range_inclusive! { i8, i16, i32, i64, i128, isize }

// #[cfg(test)]
// mod tests {
//     use crate::prelude::*;

//     // TODO: have a cooler name
//     #[test]
//     fn test_with_cool_name() {
//         assert_eq!(
//             run_test(|_| false, -42..=6, &mut crate::rand::thread_rng())
//                 .unwrap_err()
//                 .simplest_failing_input,
//             0
//         );

//         assert_eq!(
//             run_test(|n| n < 123, -45..=1000i64, &mut crate::rand::thread_rng())
//                 .unwrap_err()
//                 .simplest_failing_input,
//             123
//         );

//         assert_eq!(
//             run_test(
//                 |_| false,
//                 i128::MIN..=i128::MAX,
//                 &mut crate::rand::thread_rng()
//             )
//             .unwrap_err()
//             .simplest_failing_input,
//             0
//         );

//         assert_eq!(
//             run_test(|n| n > -100, -421..=-21i32, &mut crate::rand::thread_rng())
//                 .unwrap_err()
//                 .simplest_failing_input,
//             -100
//         );

//         assert_eq!(
//             run_test(
//                 |n| n < 643,
//                 45..=2000000i128,
//                 &mut crate::rand::thread_rng()
//             )
//             .unwrap_err()
//             .simplest_failing_input,
//             643
//         );

//         assert_eq!(
//             run_test(|n| n > -6, i16::MIN..=3, &mut crate::rand::thread_rng())
//                 .unwrap_err()
//                 .simplest_failing_input,
//             -6
//         );
//     }
// }
