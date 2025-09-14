use crate::FormatBogusNodeRule;
use check_css_syntax::CssBogusSubSelector;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusSubSelector;
impl FormatBogusNodeRule<CssBogusSubSelector> for FormatCssBogusSubSelector {}
