use crate::prelude::*;
use check_css_syntax::{CssDeclarationWithSemicolon, CssDeclarationWithSemicolonFields};
use check_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssDeclarationWithSemicolon;
impl FormatNodeRule<CssDeclarationWithSemicolon> for FormatCssDeclarationWithSemicolon {
    fn fmt_fields(
        &self,
        node: &CssDeclarationWithSemicolon,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssDeclarationWithSemicolonFields {
            declaration,
            semicolon_token,
        } = node.as_fields();

        write!(f, [declaration.format()])?;

        if semicolon_token.is_some() {
            // if semicolon is present, use the token's format to keep the comments
            write!(f, [semicolon_token.format()])
        } else {
            write!(f, [text(";")])
        }
    }
}
