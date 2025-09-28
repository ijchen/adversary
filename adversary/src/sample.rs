use rand::Rng;

use crate::{ValueGen, n_iters::IterTwo};

/// Creates an iterator of seeds sampled from the given [`ValueGen`] attempting
/// to cover the input space as much as possible under the given constraints.
///
/// The returned iterator will yield no more than `max_total` seeds, although it
/// may yield fewer if they are still sufficient to cover the entire possible
/// input space (`.exhaustive(..)` on the given `ValueGen`).
///
/// The returned iterator will yield at least `min_random` randomly sampled
/// seeds, unless fewer are still sufficient to cover the entire input space.
///
/// # Panics
/// if `max_total < min_random`
pub fn sample<G: ValueGen>(
    value_gen: &G,
    max_total: usize,
    min_random: usize,
    rng: &mut (impl Rng + ?Sized),
) -> impl Iterator<Item = G::Seed> {
    assert!(
        min_random <= max_total,
        "can't sample at least {min_random} random seeds without going over a total of {max_total} seeds"
    );

    // Ideally, we try every possible seed (if it's within `max_total`)
    if value_gen
        .cardinality()
        .is_some_and(|cardinality| cardinality <= max_total)
    {
        return IterTwo::A(value_gen.exhaustive());
    }

    // If we can't do every possible seed, do as many adversarial values as we
    // can while still including at least `min_random` random values.
    // (this can't overflow - we assert `min_random <= max_total` above)
    let max_adversarial = max_total - min_random;
    IterTwo::B(
        value_gen
            .adversarial()
            .take(max_adversarial)
            .chain(std::iter::repeat_with(|| value_gen.sample(rng)))
            .take(max_total),
    )
}
