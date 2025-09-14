use crate::prelude::*;
use check_css_syntax::{CssPropertyAtRule, CssPropertyAtRuleFields};
use check_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPropertyAtRule;
impl FormatNodeRule<CssPropertyAtRule> for FormatCssPropertyAtRule {
    fn fmt_fields(&self, node: &CssPropertyAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let CssPropertyAtRuleFields {
            property_token,
            name,
            block,
        } = node.as_fields();

        write!(
            f,
            [
                property_token.format(),
                space(),
                name.format(),
                space(),
                block.format()
            ]
        )
    }
}
