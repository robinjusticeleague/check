use crate::prelude::*;
use check_css_syntax::{CssContainerAndQuery, CssContainerAndQueryFields};
use check_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssContainerAndQuery;
impl FormatNodeRule<CssContainerAndQuery> for FormatCssContainerAndQuery {
    fn fmt_fields(&self, node: &CssContainerAndQuery, f: &mut CssFormatter) -> FormatResult<()> {
        let CssContainerAndQueryFields {
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
