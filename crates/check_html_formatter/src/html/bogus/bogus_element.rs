use crate::FormatBogusNodeRule;
use check_html_syntax::HtmlBogusElement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlBogusElement;
impl FormatBogusNodeRule<HtmlBogusElement> for FormatHtmlBogusElement {}
