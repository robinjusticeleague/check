use crate::prelude::*;
use check_css_syntax::{CssQueryFeatureRangeInterval, CssQueryFeatureRangeIntervalFields};
use check_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssQueryFeatureRangeInterval;
impl FormatNodeRule<CssQueryFeatureRangeInterval> for FormatCssQueryFeatureRangeInterval {
    fn fmt_fields(
        &self,
        node: &CssQueryFeatureRangeInterval,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssQueryFeatureRangeIntervalFields {
            left,
            left_comparison,
            name,
            right_comparison,
            right,
        } = node.as_fields();

        write!(
            f,
            [
                left.format(),
                space(),
                left_comparison.format(),
                space(),
                name.format(),
                space(),
                right_comparison.format(),
                space(),
                right.format()
            ]
        )
    }
}
