mod never_shrink;
mod shrinker;

// TODO(ijchen): I think I want to publicly expose this module, probably as
// `shrinking` or something.

pub use never_shrink::NeverShrink;
pub use shrinker::Shrinker;
