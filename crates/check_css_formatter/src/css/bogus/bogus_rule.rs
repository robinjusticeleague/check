use crate::FormatBogusNodeRule;
use check_css_syntax::CssBogusRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusRule;
impl FormatBogusNodeRule<CssBogusRule> for FormatCssBogusRule {}
