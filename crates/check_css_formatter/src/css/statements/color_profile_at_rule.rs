use crate::prelude::*;
use check_css_syntax::{CssColorProfileAtRule, CssColorProfileAtRuleFields};
use check_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssColorProfileAtRule;
impl FormatNodeRule<CssColorProfileAtRule> for FormatCssColorProfileAtRule {
    fn fmt_fields(&self, node: &CssColorProfileAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let CssColorProfileAtRuleFields {
            color_profile_token,
            name,
            block,
        } = node.as_fields();

        write!(
            f,
            [
                color_profile_token.format(),
                space(),
                name.format(),
                space(),
                block.format()
            ]
        )
    }
}
