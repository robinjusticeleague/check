use crate::prelude::*;
use check_formatter::write;

use crate::utils::FormatStatementSemicolon;

use check_js_syntax::JsBreakStatement;
use check_js_syntax::JsBreakStatementFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsBreakStatement;

impl FormatNodeRule<JsBreakStatement> for FormatJsBreakStatement {
    fn fmt_fields(&self, node: &JsBreakStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsBreakStatementFields {
            break_token,
            label,
            semicolon_token,
        } = node.as_fields();

        write!(f, [break_token.format()])?;

        if let Some(label) = &label {
            write!(f, [space(), label.format()])?;
        }

        write!(f, [FormatStatementSemicolon::new(semicolon_token.as_ref())])
    }
}
