use crate::prelude::*;
use check_css_syntax::{CssContainerStyleNotQuery, CssContainerStyleNotQueryFields};
use check_formatter::write;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssContainerStyleNotQuery;
impl FormatNodeRule<CssContainerStyleNotQuery> for FormatCssContainerStyleNotQuery {
    fn fmt_fields(
        &self,
        node: &CssContainerStyleNotQuery,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssContainerStyleNotQueryFields { not_token, query } = node.as_fields();

        write!(f, [not_token.format(), space(), query.format()])
    }
}
