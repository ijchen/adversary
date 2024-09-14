use crate::{Canonical, InputGenerator};

impl Canonical for bool {
    fn canonical() -> impl InputGenerator<Input = Self> {
        crate::bool::chance(0.5, false)
    }
}
