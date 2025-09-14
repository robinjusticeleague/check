use crate::prelude::*;
use check_css_syntax::{CssViewTransitionAtRule, CssViewTransitionAtRuleFields};
use check_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssViewTransitionAtRule;
impl FormatNodeRule<CssViewTransitionAtRule> for FormatCssViewTransitionAtRule {
    fn fmt_fields(&self, node: &CssViewTransitionAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let CssViewTransitionAtRuleFields {
            view_transition_token,
            block,
        } = node.as_fields();

        write!(f, [view_transition_token.format(), space(), block.format()])
    }
}
