pub struct TestConfig {
    /// The maximum number of attempts to find a failing value before giving up.
    ///
    /// Note that fewer attempts may be made, particularly when all values may
    /// be exhaustively checked in fewer attempts.
    pub max_attempts: usize,

    /// The minimum number of randomized attempts to find a failing value.
    ///
    /// Note that no randomized attempts will be made if all values can be
    /// exhaustively checked within `max_attempts`.
    pub min_randomized_attempts: usize,

    /// The name of the test, if available.
    pub test_name: Option<String>,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            max_attempts: 1_000_000,
            min_randomized_attempts: 200_000,
            test_name: None,
        }
    }
}
