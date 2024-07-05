use crate::{Adversarial, Exhaustive, Sample, Shrink};

use super::Canonical;

impl<T> Exhaustive<Option<T>> for Canonical
where
    Canonical: Exhaustive<T>,
{
    fn cardinality(&self) -> Option<usize> {
        usize::checked_add(Self::cardinality(&self)?, 1)
    }

    fn exhaustive(&self) -> impl Iterator<Item = Option<T>> {
        std::iter::once(None).chain(Self::exhaustive(&self).map(Some))
    }
}

impl<T> Adversarial<Option<T>> for Canonical
where
    Canonical: Adversarial<T>,
{
    // TODO(ichen): I'd really like a way to ensure None is always tested, even
    // if there's too many adversarial inputs for Some(T) to use
    fn adversarial_count(&self) -> Option<usize> {
        usize::checked_add(Self::adversarial_count(&self)?, 1)
    }

    fn adversarial(&self) -> impl Iterator<Item = Option<T>> {
        std::iter::once(None).chain(Self::adversarial(&self).map(Some))
    }
}

impl<T> Sample<Option<T>> for Canonical
where
    Canonical: Sample<T>,
{
    fn sample(&self, rng: &mut impl rand::Rng) -> Option<T> {
        // TODO(ichen): is 50/50 Some/None the best way to do this?
        rng.gen::<bool>().then(|| self.sample(rng))
    }
}

#[derive(Debug)]
enum OptionHistoryInner<T> {
    NoneFailed,
    WithHistory {
        history: T,
        none_tried_and_passed: bool,
    },
}

#[derive(Debug)]
pub struct OptionHistory<T> {
    inner: OptionHistoryInner<T>,
}

// TODO: do a much more thoughtful and improved implementation - this is just a
// basic impl to get started with.
impl<T> Shrink<Option<T>> for Canonical
where
    Canonical: Shrink<T>,
{
    type History = OptionHistory<<Canonical as Shrink<T>>::History>;

    fn history_from_failure(&self, failing_input: &Option<T>) -> Self::History {
        OptionHistory {
            inner: match failing_input {
                Some(failing_input) => OptionHistoryInner::WithHistory {
                    history: self.history_from_failure(failing_input),
                    none_tried_and_passed: false,
                },

                None => OptionHistoryInner::NoneFailed,
            },
        }
    }

    fn generate_report_details(&self, history: Self::History) -> String {
        match history.inner {
            OptionHistoryInner::NoneFailed => format!("TODO (None failed)"),

            OptionHistoryInner::WithHistory {
                history,
                none_tried_and_passed: true,
            } => format!(
                "TODO (None passed, more info: {})",
                self.generate_report_details(history)
            ),

            OptionHistoryInner::WithHistory {
                none_tried_and_passed: false,
                ..
            } => unreachable!(),
        }
    }

    fn update_history(&self, history: &mut Self::History, input: &Option<T>, test_passed: bool) {
        match (&mut history.inner, input, test_passed) {
            // None already failed, so we wouldn't try anything again. If we
            // somehow did, just ignore the result and stay as NoneFailed
            (OptionHistoryInner::NoneFailed, _, _) => (),

            // None just failed - drop our old history, we only care about None
            (_, None, false) => history.inner = OptionHistoryInner::NoneFailed,

            // We just tried None and it passed - remember that
            (
                OptionHistoryInner::WithHistory {
                    none_tried_and_passed,
                    ..
                },
                None,
                true,
            ) => *none_tried_and_passed = true,

            // We have some new information about the Some(...) case
            (OptionHistoryInner::WithHistory { history, .. }, Some(input), test_passed) => {
                self.update_history(history, input, test_passed)
            }
        }
    }

    fn next_input(&self, rng: &mut impl rand::Rng, history: &Self::History) -> Option<Option<T>> {
        match &history.inner {
            // If we already know None failed, we're done shrinking
            OptionHistoryInner::NoneFailed => None,

            // If we haven't tested None yet, try it
            OptionHistoryInner::WithHistory {
                none_tried_and_passed: false,
                ..
            } => Some(None),

            // None passes, try to shrink the Some(...) value
            OptionHistoryInner::WithHistory {
                none_tried_and_passed: true,
                history,
            } => Some(Some(self.next_input(rng, history)?)),
        }
    }
}
