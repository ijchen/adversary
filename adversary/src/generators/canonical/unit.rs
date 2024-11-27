use crate::{just, Canonical, InputGenerator};

impl Canonical for () {
    fn canonical() -> impl InputGenerator<Input = Self> {
        just(())
    }
}
