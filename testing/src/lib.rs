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
}
