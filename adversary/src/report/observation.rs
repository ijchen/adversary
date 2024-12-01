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

#[derive(Debug, Clone)]
pub enum Importance {
    Important,           // Always displayed to the user
    MaybeRelevant,       // Sometimes displayed to the user
    ProbablyUnimportant, // Never displayed to the user
}
