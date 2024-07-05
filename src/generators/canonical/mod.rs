use crate::InputGenerator;

mod bool;
mod float;
mod ints;
mod unit;

/// A struct that acts as a canonical input generator for many types.
#[derive(Debug)]
struct Canonical;

#[allow(private_bounds)] // TODO: is this desirable?
pub fn any<T>() -> impl InputGenerator<T>
where
    Canonical: InputGenerator<T>,
    T: Clone,
{
    Canonical
}
