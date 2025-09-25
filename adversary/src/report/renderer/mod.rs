mod plaintext;

use super::Report;

pub use plaintext::Plaintext;

pub trait ReportRenderer {
    // TODO(ijchen): add a Config type
    type Output;
    // TODO(ijchen): consider Converted<'_> with lifetime-GAT that borrows from T
    // TODO(ijchen): the name "Convnerted" kinda sucks
    type Converted;

    fn render<T>(report: &Report<T>, converter: impl Fn(&T) -> Self::Converted) -> Self::Output;
}
