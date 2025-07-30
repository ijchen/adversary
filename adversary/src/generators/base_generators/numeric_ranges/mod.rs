// TODO(ichen): Update this comment, the conflicting trait impl is the real
// issue here - a blanket impl for any `T: RangeBounds` would mean anything
// which does *or could possibly ever* implement `RangeBounds` would not be
// allowed to have `IntoValueGen` implemented for it, directly or through
// another impl.
// NOTE(ichen): I don't think I can (efficiently) implement this generically for
// any `T: std::ops::RangeBounds`. When we need to iterate over the range, the
// concrete type of the iterator is different depending on what kind of `Bound`
// we're working with, and we're often returning that (those) type(s) as an RPIT
// `impl Iterator<Item = ...>`. In order to do that, the only solution I can
// think of is to use `Box<dyn Iterator<Item = ...>>`, but I have low confidence
// that the compiler will be able to optimize through the heap indirection.
//
// There's also possibly some conflicting trait implementation / specialization
// issues - `std::ops::RangeBounds` is a foreign trait, so nothing's stopping
// std from implementing it on one of the other foreign types (&[T], String,
// etc.) I've manually implemented ValueGen for.
//
// If anybody can think of a way to impl ValueGen for (impl RangeBounds) in a
// zero-cost way that avoids trait conflict issues, I'd absolutely love to see
// what you're cooking up. Maybe some newtype enum dispatch thing?

mod signed_int;
mod unsigned_int;

// TODO: expose these fully publicly through `crate::shrinker`
#[expect(
    unused,
    reason = "gonna move these to a different location soon, want to remember signed as well"
)]
pub use signed_int::RangeInclusiveShrinkerSigned;
pub use unsigned_int::RangeInclusiveShrinkerUnsigned;

/// A shared struct across all numeric range types for value generation.
// Representation invariant: min <= max
// TODO: this should not be pub, make private once ATPIT allows IntoValueGen
// impls to hide the concrete type of IntoValueGen::Gen
pub struct RangeInclusiveGen<T> {
    /// The (inclusive) minimum value in the range of allowable values
    min: T,

    /// The (inclusive) maximum value in the range of allowable values
    max: T,
}
