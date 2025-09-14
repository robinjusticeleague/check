use crate::prelude::*;

use check_formatter::write;
use check_js_syntax::JsTryStatement;
use check_js_syntax::JsTryStatementFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsTryStatement;

impl FormatNodeRule<JsTryStatement> for FormatJsTryStatement {
    fn fmt_fields(&self, node: &JsTryStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsTryStatementFields {
            try_token,
            body,
            catch_clause,
        } = node.as_fields();

        write![
            f,
            [
                try_token.format(),
                space(),
                body.format(),
                space(),
                catch_clause.format(),
            ]
        ]
    }
}
