mod never_shrink;
mod shrinker;

// TODO(ijchen): I think I want to publicly expose this module, probably as
// `shrinking` or something.

// TODO: generic shrinker that wraps another shrinker but just adds a step at
// the end that runs the final failing inputs many times to determine if it
// fails reliably, or if some percentage of the time it actually passes

pub use never_shrink::NeverShrink;
pub use shrinker::Shrinker;
