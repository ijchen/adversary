#[derive(Debug, Clone)]
pub struct Observation {
    pub contents: String,
    pub importance: Importance,
}

impl Observation {
    pub fn new(contents: impl Into<String>, importance: Importance) -> Self {
        Self {
            contents: contents.into(),
            importance,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Importance {
    /// An important observation - always displayed to the user
    Important,

    /// A possibly relevant observation - sometimes displayed to the user
    #[default]
    MaybeRelevant,

    /// A probably unimportant observation - rarely displayed to the user
    ProbablyUnimportant,
}
