use crate::prelude::*;
use check_formatter::write;
use check_grit_syntax::{GritSnippetRegexLiteral, GritSnippetRegexLiteralFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritSnippetRegexLiteral;
impl FormatNodeRule<GritSnippetRegexLiteral> for FormatGritSnippetRegexLiteral {
    fn fmt_fields(
        &self,
        node: &GritSnippetRegexLiteral,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        let GritSnippetRegexLiteralFields { value_token } = node.as_fields();
        write!(f, [value_token.format()])
    }
}
