use crate::prelude::*;
use check_css_syntax::{CssSupportsNotCondition, CssSupportsNotConditionFields};
use check_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSupportsNotCondition;
impl FormatNodeRule<CssSupportsNotCondition> for FormatCssSupportsNotCondition {
    fn fmt_fields(&self, node: &CssSupportsNotCondition, f: &mut CssFormatter) -> FormatResult<()> {
        let CssSupportsNotConditionFields { not_token, query } = node.as_fields();

        write!(f, [not_token.format(), space(), query.format()])
    }
}
