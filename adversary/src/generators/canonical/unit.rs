use crate::{Canonical, ValueGen, just};

impl Canonical for () {
    fn canonical() -> impl ValueGen<Value = Self> {
        just(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{prelude::*, self_test_helpers::check_impls_canonical};

    // There's not much to test here - since unit is... well... unit. Traits
    // being implemented and a lack of panicking is basically all we can test
    // for.
    #[test]
    fn unit_canonical_sanity() {
        check_impls_canonical::<()>();

        // Check that the canonical () generator doesn't panic
        run_test(|()| true, any(), &mut adv::rand::thread_rng()).unwrap();
    }
}
