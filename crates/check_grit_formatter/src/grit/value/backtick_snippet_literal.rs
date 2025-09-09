use crate::prelude::*;
use check_formatter::write;
use check_grit_syntax::{GritBacktickSnippetLiteral, GritBacktickSnippetLiteralFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritBacktickSnippetLiteral;
impl FormatNodeRule<GritBacktickSnippetLiteral> for FormatGritBacktickSnippetLiteral {
    fn fmt_fields(
        &self,
        node: &GritBacktickSnippetLiteral,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        let GritBacktickSnippetLiteralFields { value_token } = node.as_fields();
        write!(f, [value_token.format()])
    }
}
