use crate::{Canonical, ValueGen};

use super::any;

impl<T: Canonical> Canonical for Vec<T> {
    fn canonical() -> impl ValueGen<Value = Self> {
        // TODO: do this for real, with non-uniform distribution (bucketed
        // exponential-ish decay)
        let len_range = 0..=match std::mem::size_of::<T>() {
            0 => usize::MAX,
            1 => 8192,
            2..4 => 2048,
            4..8 => 1024,
            8..64 => 512,
            64..1024 => 128,
            1024..4096 => 64,
            4096.. => 16,
        };
        crate::vec::VecValueGen::new(any(), len_range)
    }
}
