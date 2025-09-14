use crate::FormatBogusNodeRule;
use check_css_syntax::CssBogusSupportsCondition;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusSupportsCondition;
impl FormatBogusNodeRule<CssBogusSupportsCondition> for FormatCssBogusSupportsCondition {}
