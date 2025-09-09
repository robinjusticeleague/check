use crate::prelude::*;
use check_css_syntax::{CssMediaAndTypeQuery, CssMediaAndTypeQueryFields};
use check_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssMediaAndTypeQuery;
impl FormatNodeRule<CssMediaAndTypeQuery> for FormatCssMediaAndTypeQuery {
    fn fmt_fields(&self, node: &CssMediaAndTypeQuery, f: &mut CssFormatter) -> FormatResult<()> {
        let CssMediaAndTypeQueryFields {
            left,
            and_token,
            right,
        } = node.as_fields();

        write!(
            f,
            [
                left.format(),
                space(),
                and_token.format(),
                space(),
                right.format()
            ]
        )
    }
}
