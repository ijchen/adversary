pub fn add(lhs: i32, rhs: i32) -> i32 {
    i32::checked_add(lhs, rhs).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use adversary::prelude::*;

    #[adv_test]
    fn add_commutative(a: i32, b: i32) -> bool {
        i32::checked_add(a, b).is_none() || add(a, b) == add(b, a)
    }

    #[adv_test]
    fn add_identity(n: i32) -> bool {
        add(n, 0) == n
    }

    #[adv_test]
    fn add_associative(a: i32, b: i32, c: i32) -> bool {
        if i32::checked_add(a, b).is_none_or(|ab| i32::checked_add(ab, c).is_none())
            || i32::checked_add(b, c).is_none_or(|bc| i32::checked_add(a, bc).is_none())
        {
            return true;
        }

        add(add(a, b), c) == add(a, add(b, c))
    }

    #[adv_test(b = 100..1209451)]
    fn custom_gen((a, b, c): (u32, u32, u32)) -> bool {
        let _ = (a, c);
        (100..1209451).contains(&b)
    }

    // #[adv_test]
    // fn vec_test(list: Vec<i32>) -> bool {
    //     list.iter()
    //         .copied()
    //         .try_fold(0i32, |accum, elem| accum.checked_add(elem))
    //         .is_none_or(|n| n % 10 != 9)
    // }

    #[adv_test]
    fn vec_test(list: Vec<u32>) -> bool {
        list.iter()
            .copied()
            .try_fold(0u32, |accum, elem| accum.checked_add(elem))
            .is_none_or(|n| n % 10 != 9 || n < 1000)
    }

    // #[derive(Clone)]
    // struct Foo;
    // impl adv::Canonical for Foo {
    //     fn canonical() -> impl ValueGen<Value = Self> + Send + Sync + Unpin {
    //         just(Foo)
    //     }
    // }

    // #[adv_test]
    // fn my_cool_test1(_: i32, _: bool) -> bool {
    //     false
    // }
    // #[adv_test]
    // fn my_cool_test2(_: i32) -> bool {
    //     false
    // }
    // #[adv_test]
    // fn my_cool_test3(_: Foo) -> bool {
    //     false
    // }

    // #[derive(Clone)]
    // struct Nothing;
    // impl adv::Canonical for Nothing {
    //     fn canonical() -> impl ValueGen<Value = Self> + Send + Sync + Unpin {
    //         just(Nothing)
    //     }
    // }
    // #[derive(Debug, Clone)]
    // struct Debugger;
    // impl adv::Canonical for Debugger {
    //     fn canonical() -> impl ValueGen<Value = Self> + Send + Sync + Unpin {
    //         just(Debugger)
    //     }
    // }
    // #[derive(Clone)]
    // struct Displayer;
    // impl adv::Canonical for Displayer {
    //     fn canonical() -> impl ValueGen<Value = Self> + Send + Sync + Unpin {
    //         just(Displayer)
    //     }
    // }
    // impl std::fmt::Display for Displayer {
    //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //         write!(f, "My cool Displayer!!!")
    //     }
    // }
    // #[adv_test]
    // fn my_cool_test4(_: Displayer, _: Debugger, _: Nothing) -> bool {
    //     false
    // }

    // #[adv_test]
    // fn my_test(a: u64, b: u64, c: u64) -> Result<(), String> {
    //     if b == 0 || a == c || a == b || b < 10 {
    //         return Ok(());
    //     }

    //     if a % b == c {
    //         return Err(format!("Failed, {a} % {b} == {c}"));
    //     }

    //     Ok(())
    // }
}
