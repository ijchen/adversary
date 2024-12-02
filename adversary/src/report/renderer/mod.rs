mod plaintext;

use super::Report;

pub use plaintext::Plaintext;

// TODO: split this into its own submodule
pub trait ReportRenderer {
    type Output;
    type ConvertedT;

    fn render<T>(report: &Report<T>, convert: impl Fn(&T) -> Self::ConvertedT) -> Self::Output;
}
