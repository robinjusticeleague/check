use crate::FormatBogusNodeRule;
use check_css_syntax::CssBogusSelector;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusSelector;
impl FormatBogusNodeRule<CssBogusSelector> for FormatCssBogusSelector {}
