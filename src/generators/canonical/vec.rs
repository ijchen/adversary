use crate::{Adversarial, Exhaustive, Sample, Shrink};

use super::Canonical;

impl<T: Clone> Exhaustive<Vec<T>> for Canonical
where
    Canonical: Exhaustive<T>,
{
    fn cardinality(&self) -> Option<usize> {
        None
    }

    fn exhaustive(&self) -> impl Iterator<Item = Vec<T>> {
        (0..).flat_map(|len| {
            // TODO: don't allocate unnecessarily and make this work for
            // infinite iterators
            let mut elems = vec![vec![]];

            // TODO: use with capacity (might be complicated)
            for _ in 0..len {
                elems = elems
                    .into_iter()
                    .flat_map(|v| {
                        Canonical::exhaustive(&Self).map(move |value| {
                            let mut new_v = v.clone();
                            new_v.push(value);
                            new_v
                        })
                    })
                    .collect();
            }

            elems
        })
    }
}

impl<T> Adversarial<Vec<T>> for Canonical
where
    Canonical: Adversarial<T>,
{
    fn adversarial_count(&self) -> Option<usize> {
        Some(1)
    }

    fn adversarial(&self) -> impl Iterator<Item = Vec<T>> {
        std::iter::once(Vec::new())
    }
}

impl<T> Sample<Vec<T>> for Canonical
where
    Canonical: Sample<T>,
{
    fn sample(&self, rng: &mut impl rand::Rng) -> Vec<T> {
        const LOG_RATIO: f64 = -9.491221581029905; // f64::ln(1.0 - (CHANCE = 0.10)).recip();
        let length = f64::min(
            f64::floor(LOG_RATIO * f64::ln(rng.gen_range(0.0..1.0))),
            1000.0,
            // TODO: I think `as` is defined how I want but I don't like it anyway
        ) as usize;

        std::iter::repeat_with(|| Canonical::sample(&Canonical, rng))
            .take(length)
            .collect()
    }
}

// TODO: actually implement this for real
impl<T> Shrink<Vec<T>> for Canonical
where
    Canonical: Shrink<T>,
{
    type History = ();

    fn history_from_failure(&self, _failing_input: &Vec<T>) -> Self::History {
        ()
    }

    fn generate_report_details(&self, _history: Self::History) -> String {
        "TODO: shrinking for Vec<T> is not implemented".to_string()
    }

    fn update_history(&self, _history: &mut Self::History, _input: &Vec<T>, _test_passed: bool) {}

    fn next_input(&self, _rng: &mut impl rand::Rng, _history: &Self::History) -> Option<Vec<T>> {
        None
    }
}
