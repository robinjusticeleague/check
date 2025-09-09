use crate::FormatBogusNodeRule;
use check_css_syntax::CssBogusScopeRange;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusScopeRange;
impl FormatBogusNodeRule<CssBogusScopeRange> for FormatCssBogusScopeRange {}
