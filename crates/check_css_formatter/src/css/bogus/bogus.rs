use crate::FormatBogusNodeRule;
use check_css_syntax::CssBogus;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogus;
impl FormatBogusNodeRule<CssBogus> for FormatCssBogus {}
