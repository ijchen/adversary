use crate::{Canonical, Chance, ValueGen};

impl Canonical for bool {
    fn canonical() -> impl ValueGen<Value = Self> {
        crate::bool::chance(Chance::EQUAL, false)
    }
}
