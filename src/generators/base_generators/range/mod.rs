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
// etc.) I've manually implemented InputGenerator for.
//
// If anybody can think of a way to impl InputGenerator for (impl RangeBounds)
// in a zero-cost way that avoids trait conflict issues, I'd absolutely love to
// see what you're cooking up. Maybe some newtype enum dispatch thing?

mod range;
mod range_inclusive;
