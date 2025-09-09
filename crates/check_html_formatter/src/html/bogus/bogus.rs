use crate::FormatBogusNodeRule;
use check_html_syntax::HtmlBogus;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlBogus;
impl FormatBogusNodeRule<HtmlBogus> for FormatHtmlBogus {}
