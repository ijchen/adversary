use crate::InputGenerator;

mod bool;
mod float;
mod ints;
mod unit;

/// A struct that acts as a canonical input generator for many types.
// NOTE(ichen): I could derive a lot more, but I haven't decided if I want to
// make those API promises yet. This is the same reason the struct has it's ZST
// field - I don't want to make an API promise about its contents.
#[derive(Debug)]
pub struct Canonical {
    _non_public: (),
}

pub fn any<T>() -> impl InputGenerator<T>
where
    Canonical: InputGenerator<T>,
    T: Clone,
{
    Canonical { _non_public: () }
}
