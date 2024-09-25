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
}
