use crate::{Canonical, InputGenerator, IntoInputGenerator};

macro_rules! canonical_tuples {
    ($( ( $($letter:ident),+$(,)? ) ),*$(,)?) => {$(
        impl<$($letter : Canonical ,)+> Canonical for ($($letter ,)+) {
            fn canonical() -> impl InputGenerator<Input = Self> {
                ($(<$letter>::canonical() ,)+).into_input_generator()
            }
        }
    )*}
}

canonical_tuples! {
    // TODO(ichen): uncomment after implementing unary and binary tuple gens
    // (A,),
    // (A, B),
    (A, B, C),
    (A, B, C, D),
    (A, B, C, D, E),
    (A, B, C, D, E, F),
    (A, B, C, D, E, F, G),
    (A, B, C, D, E, F, G, H),
    (A, B, C, D, E, F, G, H, I),
    (A, B, C, D, E, F, G, H, I, J),
    (A, B, C, D, E, F, G, H, I, J, K),
    (A, B, C, D, E, F, G, H, I, J, K, L),
}
