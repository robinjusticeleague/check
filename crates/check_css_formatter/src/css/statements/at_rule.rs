use crate::prelude::*;
use check_css_syntax::{CssAtRule, CssAtRuleFields};
use check_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssAtRule;
impl FormatNodeRule<CssAtRule> for FormatCssAtRule {
    fn fmt_fields(&self, node: &CssAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let CssAtRuleFields { at_token, rule } = node.as_fields();

        write!(f, [at_token.format(), rule.format()])
    }
}
