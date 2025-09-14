use crate::FormatBogusNodeRule;
use check_css_syntax::CssBogusCustomIdentifier;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusCustomIdentifier;
impl FormatBogusNodeRule<CssBogusCustomIdentifier> for FormatCssBogusCustomIdentifier {}
