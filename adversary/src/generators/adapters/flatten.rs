use crate::{IntoValueGen, ValueGen, shrinker::Shrinker};

pub fn flatten<P: IntoValueGen<C>, C: IntoValueGen<T>, T>(
    parent_gen: P,
) -> impl ValueGen<Value = T> {
    Flatten(
        parent_gen.into_value_gen(),
        std::marker::PhantomData::<C::Gen>,
    )
}

// TODO: is there a way to express this that doesn't need the PhantomData hack?
struct Flatten<P, T>(P, std::marker::PhantomData<T>);

impl<P: ValueGen, C: ValueGen> ValueGen for Flatten<P, C>
where
    P::Value: IntoValueGen<C::Value, Gen = C>,
{
    type Value = C::Value;
    type Seed = (P::Seed, C::Seed);
    // TODO: use ATPIT once stabilized
    type Shrinker<'a>
        = crate::shrinkers::NeverShrink
    where
        Self: 'a;

    fn cardinality(&self) -> Option<usize> {
        self.0
            .exhaustive()
            .map(|parent_input_source| {
                self.0
                    .create_value(parent_input_source)
                    .into_value_gen()
                    .cardinality()
            })
            .try_fold(0usize, |total, next| total.checked_add(next?))
    }

    fn exhaustive(&self) -> impl Iterator<Item = Self::Seed> {
        self.0.exhaustive().flat_map(|parent_input_source| {
            self.0
                .create_value(parent_input_source.clone())
                .into_value_gen()
                .exhaustive()
                .map(|child_input_source| (parent_input_source.clone(), child_input_source))
                // TODO: why is this collect necessary? Very undesirable
                .collect::<Vec<_>>()
        })
    }

    fn adversarial_count(&self) -> Option<usize> {
        self.0
            .adversarial()
            .map(|parent_input_source| {
                self.0
                    .create_value(parent_input_source)
                    .into_value_gen()
                    .adversarial_count()
            })
            .try_fold(0usize, |total, next| total.checked_add(next?))
    }

    fn adversarial(&self) -> impl Iterator<Item = Self::Seed> {
        self.0.adversarial().flat_map(|parent_input_source| {
            self.0
                .create_value(parent_input_source.clone())
                .into_value_gen()
                .adversarial()
                .map(|child_input_source| (parent_input_source.clone(), child_input_source))
                // TODO: why is this collect necessary? Very undesirable
                .collect::<Vec<_>>()
        })
    }

    fn sample(&self, rng: &mut (impl rand::Rng + ?Sized)) -> Self::Seed {
        let parent_input_source = self.0.sample(rng);
        let child_input_source = self
            .0
            .create_value(parent_input_source.clone())
            .into_value_gen()
            .sample(rng);

        (parent_input_source, child_input_source)
    }

    fn new_shrinker(&self, _failing_value_seed: Self::Seed) -> Self::Shrinker<'_> {
        crate::shrinkers::NeverShrink::new() // TODO: implement flatten shrinking
    }

    fn create_value(&self, seed: Self::Seed) -> Self::Value {
        self.0
            .create_value(seed.0)
            .into_value_gen()
            .create_value(seed.1)
    }
}

enum FlattenShrinker<'a, P: ValueGen + 'a, C: ValueGen + 'a, I: Iterator<Item = C::Seed>> {
    ShrinkParent {
        parent_shrinker: P::Shrinker<'a>,
        current_child_seed: C::Seed,
        remaining_child_seeds: I,
        simplest_known_failing: (P::Seed, C::Seed),
    },
    ShrinkChild {
        parent_seed: P::Seed,
        child_shrinker: C::Shrinker<'a>,
    },
}

impl<'a, P: ValueGen, C: ValueGen, I: Iterator<Item = C::Seed>> Shrinker<(P::Seed, C::Seed)>
    for FlattenShrinker<'a, P, C, I>
{
    fn current_attempt(&self) -> Option<(P::Seed, C::Seed)> {
        match self {
            FlattenShrinker::ShrinkParent {
                parent_shrinker,
                current_child_seed,
                ..
            } => Some((
                parent_shrinker.current_attempt().unwrap(),
                current_child_seed.clone(),
            )),
            FlattenShrinker::ShrinkChild {
                parent_seed,
                child_shrinker,
            } => child_shrinker
                .current_attempt()
                .map(|child_seed| (parent_seed.clone(), child_seed)),
        }
    }

    fn update(&mut self, current_attempt_passed: bool) {
        // fn get_child_seeds() ->

        match self {
            FlattenShrinker::ShrinkParent {
                parent_shrinker,
                current_child_seed,
                remaining_child_seeds,
                simplest_known_failing,
            } => {
                // If we found a failing value, shrink the parent
                if !current_attempt_passed {
                    *simplest_known_failing = (
                        parent_shrinker.current_attempt().unwrap(),
                        current_child_seed.clone(),
                    );
                    parent_shrinker.update(false);

                    // TODO: update current_child_seed and remaining_child_seeds
                    todo!()
                }
                // If we didn't find a failing value, try the next one (if there
                // is another to try)
                else {
                    match remaining_child_seeds.next() {
                        Some(next) => *current_child_seed = next,
                        // If we're out of child seeds to try, parent shrinking
                        // should be informed of a "test pass" (we couldn't find
                        // a failing case)
                        None => {
                            parent_shrinker.update(true);

                            // If the parent shrinker still has more seeds to
                            // try, keep going with a new set of child seeds
                            if parent_shrinker.current_attempt().is_some() {
                                // TODO: update current_child_seed and
                                // remaining_child_seeds (and either keep going
                                // or if there's no child seed, do a parent
                                // shrinker update again. Worth noting this is
                                // not trivial to handle, make sure to correctly
                                // handle when the child shrinker immediately
                                // has no current attempt, even multiple times
                                // in a row)
                                todo!()
                            }
                            // If the parent shrinker is done, move on to
                            // shrinking the child
                            else {
                                *self = FlattenShrinker::ShrinkChild {
                                    parent_seed: simplest_known_failing.0.clone(),
                                    child_shrinker: todo!(),
                                };
                            }
                        }
                    }
                }
            }
            FlattenShrinker::ShrinkChild { child_shrinker, .. } => {
                child_shrinker.update(current_attempt_passed)
            }
        }
    }

    fn into_observations(self) -> Vec<crate::report::Observation> {
        // TODO(ichen): useful observations
        Vec::new()
    }
}
