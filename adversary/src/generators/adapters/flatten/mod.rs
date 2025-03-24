use flatten::Flatten;

use crate::{IntoValueGen, ValueGen};

mod exhaustive_owner;
mod flatten;
mod flatten_shrinker;
mod shrink_child;
mod shrink_parent;
mod shrinker_owner;

pub fn flatten<P: IntoValueGen<C>, C: IntoValueGen<T>, T>(
    parent_gen: P,
) -> impl ValueGen<Value = T> {
    // TODO: call .into_value_gen(..) here instead of in Flatten::new(..)
    Flatten::new(parent_gen)
}
