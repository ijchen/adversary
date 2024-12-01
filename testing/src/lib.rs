#[cfg(test)]
mod tests {
    use adversary::prelude::*;

    // #[adv_test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }

    #[adv_test]
    fn foo(a: bool) {
        assert!(a || !a);
    }

    #[adv_test]
    fn bar(a: bool, _b: (), c: bool, _d: u8, _e: (bool, u8, (i128, (), ()), u32)) {
        assert!(a || c || true);
    }
}
