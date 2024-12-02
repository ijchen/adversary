mod plaintext;

use super::Report;

pub use plaintext::Plaintext;

pub trait ReportRenderer {
    type Output;
    type Converted;

    fn render<T>(report: &Report<T>, converter: impl Fn(&T) -> Self::Converted) -> Self::Output;
}
