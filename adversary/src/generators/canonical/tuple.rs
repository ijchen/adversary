use crate::{Canonical, IntoValueGen, ValueGen};

macro_rules! canonical_tuples {
    ($( ( $($letter:ident),+$(,)? ) ),*$(,)?) => {$(
        impl<$($letter : Canonical ,)+> Canonical for ($($letter ,)+) {
            fn canonical() -> impl ValueGen<Value = Self> {
                ($(<$letter>::canonical() ,)+).into_value_gen()
            }
        }
    )*}
}

canonical_tuples! {
    (A,),
    (A, B),
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
