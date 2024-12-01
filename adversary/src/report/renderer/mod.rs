mod plaintext;

use super::Report;

pub use plaintext::Plaintext;

// TODO: split this into its own submodule
pub trait ReportRenderer {
    type Output;

    fn render<T>(report: &Report<T>) -> Self::Output;
}
