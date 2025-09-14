use crate::FormatBogusNodeRule;
use check_css_syntax::CssBogusBlock;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusBlock;
impl FormatBogusNodeRule<CssBogusBlock> for FormatCssBogusBlock {}
