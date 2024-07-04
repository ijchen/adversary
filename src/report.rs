#[derive(Debug, Clone)]
pub struct Report<T> {
    pub original_failing_input: T,
    pub minimal_failing_input: Option<T>,
    pub details: String,
}
