use crate::prelude::*;

use crate::utils::FormatStatementBody;
use check_formatter::write;
use check_js_syntax::JsElseClause;
use check_js_syntax::JsElseClauseFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsElseClause;

impl FormatNodeRule<JsElseClause> for FormatJsElseClause {
    fn fmt_fields(&self, node: &JsElseClause, f: &mut JsFormatter) -> FormatResult<()> {
        use check_js_syntax::AnyJsStatement::*;

        let JsElseClauseFields {
            else_token,
            alternate,
        } = node.as_fields();

        let alternate = alternate?;

        write!(
            f,
            [
                else_token.format(),
                group(
                    &FormatStatementBody::new(&alternate)
                        .with_forced_space(matches!(alternate, JsIfStatement(_)))
                )
            ]
        )
    }
}
