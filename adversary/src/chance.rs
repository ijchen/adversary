use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy, PartialEq)]
// Invariant: 0.0 <= self.0 <= 1.0
pub struct Chance(f64);

// NOTE(ijchen): it is a representation invariant that self.0 is in [0.0, 1.0]
impl Eq for Chance {}
impl Ord for Chance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        PartialOrd::partial_cmp(&self.0, &other.0)
            .expect("chance's inner value should be in [0.0, 1.0] - this is a bug")
    }
}
impl PartialOrd for Chance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Hash for Chance {
    fn hash<H: Hasher>(&self, state: &mut H) {
        assert!(
            (0.0..=1.0).contains(&self.0),
            "chance's inner value should be in [0.0, 1.0] - this is a bug"
        );

        state.write_u64(self.0.to_bits());
    }
}

impl Chance {
    pub const IMPOSSIBLE: Self = Self(0.0);
    pub const EQUAL: Self = Self(0.5);
    pub const GUARANTEED: Self = Self(1.0);

    pub const fn from_probability(probability: f64) -> Option<Self> {
        if probability.is_nan() || probability < 0.0 || probability > 1.0 {
            return None;
        }

        // NOTE(ijchen): `.abs()` to normalize -0.0 to 0.0. I don't think this matters, but it's
        // technically observable to the user (ex. through .as_probability()), so I feel a little
        // better upholding the likely user assumption that a chance's sign is always positive.
        Some(Self(probability.abs()))
    }

    pub fn from_ratio(numerator: u64, denominator: u64) -> Option<Self> {
        Self::from_probability(numerator as f64 / denominator as f64)
    }

    pub fn from_percent(percent: f64) -> Option<Self> {
        Self::from_probability(percent / 100.0)
    }

    pub fn as_probability(self) -> f64 {
        self.0
    }

    pub fn as_percent(self) -> f64 {
        self.0 * 100.0
    }

    pub fn is_possible(self) -> bool {
        self != Self::IMPOSSIBLE
    }

    pub fn is_guaranteed(self) -> bool {
        self == Self::GUARANTEED
    }

    pub(crate) fn gen_bool(self, rng: &mut (impl crate::rand::Rng + ?Sized)) -> bool {
        rng.gen_bool(self.0)
    }
}
