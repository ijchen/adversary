use crate::{Canonical, ValueGen};

impl Canonical for bool {
    fn canonical() -> impl ValueGen<Value = Self> {
        crate::bool::chance(0.5, false)
    }
}
