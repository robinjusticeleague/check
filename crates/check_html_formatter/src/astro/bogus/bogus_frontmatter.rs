use crate::FormatBogusNodeRule;
use check_html_syntax::AstroBogusFrontmatter;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAstroBogusFrontmatter;
impl FormatBogusNodeRule<AstroBogusFrontmatter> for FormatAstroBogusFrontmatter {}
