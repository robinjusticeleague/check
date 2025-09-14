use crate::FormatBogusNodeRule;
use check_css_syntax::CssBogusAtRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusAtRule;
impl FormatBogusNodeRule<CssBogusAtRule> for FormatCssBogusAtRule {}
