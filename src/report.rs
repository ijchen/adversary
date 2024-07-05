#[derive(Debug, Clone)]
pub struct Report<T> {
    pub original_failing_input: T,
    pub shrunk_failing_input: Option<T>,
    pub details: String,
}

impl<T> Report<T> {
    pub fn minimal_failing_input(&self) -> &T {
        self.shrunk_failing_input
            .as_ref()
            .unwrap_or(&self.original_failing_input)
    }
}
