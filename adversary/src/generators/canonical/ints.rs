use crate::{Canonical, InputGenerator, IntoInputGenerator};

macro_rules! canonical_ints {
    ($($t:ty),*$(,)?) => {$(
        impl Canonical for $t {
            fn canonical() -> impl InputGenerator<Input = Self> {
                (<$t>::MIN..=<$t>::MAX).into_input_generator()
            }
        }
    )*}
}

canonical_ints! {
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
}
