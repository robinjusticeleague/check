use crate::FormatBogusNodeRule;
use check_css_syntax::CssBogusPageSelectorPseudo;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusPageSelectorPseudo;
impl FormatBogusNodeRule<CssBogusPageSelectorPseudo> for FormatCssBogusPageSelectorPseudo {}
