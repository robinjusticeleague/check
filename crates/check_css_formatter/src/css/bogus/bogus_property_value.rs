use crate::FormatBogusNodeRule;
use check_css_syntax::CssBogusPropertyValue;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusPropertyValue;
impl FormatBogusNodeRule<CssBogusPropertyValue> for FormatCssBogusPropertyValue {}
