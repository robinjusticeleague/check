use crate::FormatBogusNodeRule;
use check_css_syntax::CssBogusPseudoElement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusPseudoElement;
impl FormatBogusNodeRule<CssBogusPseudoElement> for FormatCssBogusPseudoElement {}
