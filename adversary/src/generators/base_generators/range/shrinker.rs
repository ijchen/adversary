use crate::{report::Observation, shrinker::Shrinker};

// Shrinking steps (`min` is the minimum value in the range, `k` is the current
// minimal known failing input): (TODO: not yet implemented)
// - Try `min` right away
//   - If `min` is found to be failing, shrinking ends immediately
//   - The goal behind this step is to waste no time trying larger values if the
//     minimal value will fail anyway
// - Binary search towards `min`
//   - The goal behind this step is to quickly reduce `k` as much as possible
// - Try 12 equally-distributed values between `min` and `k - 100`
//   - If this step finds a new minimal failing value, we start back from the
//     binary search step
//   - If `k - 100 - (min + 1) < 12`, this step is skipped
//   - The goal behind this step is to sample many spread out points in the
//     remaining input space between `min` and `k`, with the hope that we will
//     catch any "pockets" of failing values that binary search undershot
// - Try every number in the range `MAX(min + 1, k - 100)..=(k - 1)`
//   - If this step finds a new simplest failing value, we start back from the
//     binary search step
//   - The goal behind this step is to try a large run of consecutive values
//     just under `k`, with the hope that we will be able to recognize and jump
//     past any relatively small gaps of passing inputs between `k` and simpler
//     failing values
pub struct RangeInclusiveShrinkerUnsigned<T> {
    min: T,
    simplest_known_failing: T,
    phase: Phase<T>,
}

const SPREAD_COUNT: u8 = 12;
const CONSEC_COUNT: u8 = 100;

enum Phase<T> {
    // Invariant: self.simplest_known_failing != self.min
    TryMin,

    // Low is the minimum possibly failing value, high is the maximum known
    // failing value
    // Invariant: low < high
    BinarySearch { low: T, high: T },

    // Invariant: 0 <= index < SPREAD_COUNT
    Spread { index: u8 },

    Consecutive { current: T },

    Done,
}

macro_rules! shrinker {
    ($($t: ty),+$(,)?) => {$(
        impl RangeInclusiveShrinkerUnsigned<$t> {
            pub fn new(min: $t, max: $t, simplest_known_failing: $t) -> Self {
                assert!(min <= max);
                assert!(min <= simplest_known_failing);
                assert!(simplest_known_failing <= max);

                Self {
                    min,
                    simplest_known_failing,
                    phase: if simplest_known_failing == min {
                        Phase::Done
                    } else {
                        Phase::TryMin
                    },
                }
            }
        }

        impl Shrinker for RangeInclusiveShrinkerUnsigned<$t> {
            type InputSource = $t;

            fn current_attempt(&self) -> Option<Self::InputSource> {
                match self.phase {
                    Phase::TryMin => Some(self.min),
                    Phase::BinarySearch { low, high } => Some((high - low) / 2 + low),
                    Phase::Spread { index } => {
                        // Both inclusive ends of the range of values to spread
                        // out within
                        let lowest = self.min + 1;
                        let highest = self.simplest_known_failing - <$t>::from(CONSEC_COUNT) - 1;
                        let num_of_values_in_range = highest - lowest + 1;

                        // TODO(ichen): is division truncation too much of an
                        // issue here? Can it round to unreasonable things?
                        let start: $t = lowest + num_of_values_in_range / <$t>::from(SPREAD_COUNT * 2);
                        let offset: $t = num_of_values_in_range / <$t>::from(SPREAD_COUNT) * <$t>::from(index);

                        Some(start + offset)
                    },
                    Phase::Consecutive { current } => Some(current),
                    Phase::Done => None,
                }
            }

            fn update(&mut self, current_attempt_passed: bool) {
                let current_attempt = self.current_attempt();
                match &mut self.phase {
                    Phase::TryMin => {
                        // If the min value passed, move on to the next step
                        self.phase = if current_attempt_passed {
                            let low = self.min + 1;
                            let high = self.simplest_known_failing;

                            // If there's nothing lower than the simplest known
                            // failing value to try, we're done shrinking
                            if low == high {
                                Phase::Done
                            }
                            // Otherwise, move on to binary search
                            else {
                                Phase::BinarySearch { low, high }
                            }
                        }
                        // If the min value failed, we're done
                        else {
                            self.simplest_known_failing = self.min;
                            Phase::Done
                        }
                    }

                    Phase::BinarySearch { low, high } => {
                        // Update the high or low value in our binary search
                        if current_attempt_passed {
                            *low = current_attempt.unwrap() + 1;
                        } else {
                            *high = current_attempt.unwrap();
                            self.simplest_known_failing = *high;
                        }

                        // If we've finished the binary search...
                        if low == high {
                            let consecutive_start = self.simplest_known_failing.saturating_sub(CONSEC_COUNT.into()).max(self.min + 1);
                            // If there's not enough input space to do spread
                            // *and* consecutive without overlap, just skip
                            // spread altogether
                            self.phase = if consecutive_start.saturating_sub(SPREAD_COUNT.into()) < self.min + 1 {
                                Phase::Consecutive {
                                    current: consecutive_start,
                                }
                            }
                            // Otherwise, continue on to spread
                            else {
                                Phase::Spread { index: 0 }
                            }
                        }
                    }

                    Phase::Spread { index } => {
                        // If we found a failing value, save it and move either
                        // back to binary search, or to done
                        if !current_attempt_passed {
                            self.simplest_known_failing = self.current_attempt().unwrap();

                            // If the failing value we found is <= min + 1,
                            // there's no point binary searching - we're done.
                            self.phase = if self.simplest_known_failing <= self.min + 1 {
                                Phase::Done
                            }
                            // Otherwise, there's more values to try between min
                            // and the new simplest failing input we just found,
                            // so start back at the binary search phase with our
                            // new information
                            else {
                                Phase::BinarySearch {
                                    low: self.min + 1,
                                    high: self.simplest_known_failing,
                                }
                            };

                            return;
                        }

                        // Progress to the next value
                        *index += 1;

                        // If we've finished the spread search, move on to the
                        // consecutive phase
                        if *index == SPREAD_COUNT {
                            self.phase = Phase::Consecutive {
                                current: <$t>::max(<$t>::saturating_sub(self.simplest_known_failing, CONSEC_COUNT.into()), self.min + 1),
                            };
                        }
                    }

                    Phase::Consecutive { current } => {
                        // If we found a failing value, save it and move either
                        // back to binary search, or to done
                        if !current_attempt_passed {
                            let old_simplest_known_failing = self.simplest_known_failing;
                            self.simplest_known_failing = *current;

                            // If this consecutive run started at min + 1, we
                            // already checked everything between min and
                            // simplest_known_failing, so we're done.
                            self.phase = if <$t>::saturating_sub(old_simplest_known_failing, CONSEC_COUNT.into()) <= self.min + 1 {
                                Phase::Done
                            }
                            // Otherwise, there's more values to try between min
                            // and the new simplest failing input we just found,
                            // so start back at the binary search phase with our
                            // new information
                            else {
                                Phase::BinarySearch {
                                    low: self.min + 1,
                                    high: *current,
                                }
                            };

                            return;
                        }

                        // Progress to the next value
                        *current += 1;

                        // If we've hit the simplest known failing input, we've
                        // tried all values in our consecutive run - give up and
                        // move to done
                        if *current == self.simplest_known_failing {
                            self.phase = Phase::Done;
                        }
                    }

                    Phase::Done => { /* Nothing to do here */ }
                }
            }

            fn into_observations(self) -> Vec<Observation> {
                // TODO(ichen): useful observations
                Vec::new()
            }
        }
    )+};
}

shrinker! { u8, u16, u32, u64, u128, usize }
