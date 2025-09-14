use crate::FormatBogusNodeRule;
use check_css_syntax::CssUnknownAtRuleComponentList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssUnknownAtRuleComponentList;
impl FormatBogusNodeRule<CssUnknownAtRuleComponentList> for FormatCssUnknownAtRuleComponentList {}
