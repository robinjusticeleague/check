use crate::prelude::*;
use check_css_syntax::{CssQueryFeatureReverseRange, CssQueryFeatureReverseRangeFields};
use check_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssQueryFeatureReverseRange;
impl FormatNodeRule<CssQueryFeatureReverseRange> for FormatCssQueryFeatureReverseRange {
    fn fmt_fields(
        &self,
        node: &CssQueryFeatureReverseRange,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssQueryFeatureReverseRangeFields {
            left,
            comparison,
            right,
        } = node.as_fields();

        write!(
            f,
            [
                left.format(),
                space(),
                comparison.format(),
                space(),
                right.format()
            ]
        )
    }
}
