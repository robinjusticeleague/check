use crate::prelude::*;
use check_css_syntax::{CssContainerOrQuery, CssContainerOrQueryFields};
use check_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssContainerOrQuery;
impl FormatNodeRule<CssContainerOrQuery> for FormatCssContainerOrQuery {
    fn fmt_fields(&self, node: &CssContainerOrQuery, f: &mut CssFormatter) -> FormatResult<()> {
        let CssContainerOrQueryFields {
            left,
            or_token,
            right,
        } = node.as_fields();

        write!(
            f,
            [
                left.format(),
                space(),
                or_token.format(),
                space(),
                right.format()
            ]
        )
    }
}
