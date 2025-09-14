use crate::prelude::*;
use check_css_syntax::{CssSupportsAndCondition, CssSupportsAndConditionFields};
use check_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSupportsAndCondition;
impl FormatNodeRule<CssSupportsAndCondition> for FormatCssSupportsAndCondition {
    fn fmt_fields(&self, node: &CssSupportsAndCondition, f: &mut CssFormatter) -> FormatResult<()> {
        let CssSupportsAndConditionFields {
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
                soft_line_break_or_space(),
                right.format()
            ]
        )
    }
}
