use crate::FormatBogusNodeRule;
use check_css_syntax::CssBogusProperty;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusProperty;
impl FormatBogusNodeRule<CssBogusProperty> for FormatCssBogusProperty {}
