use crate::prelude::*;

use check_formatter::write;
use check_js_syntax::JsImportMetaExpression;
use check_js_syntax::JsImportMetaExpressionFields;
use check_js_syntax::parentheses::NeedsParentheses;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsImportMetaExpression;

impl FormatNodeRule<JsImportMetaExpression> for FormatJsImportMetaExpression {
    fn fmt_fields(&self, node: &JsImportMetaExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsImportMetaExpressionFields {
            import_token,
            dot_token,
            meta_token,
        } = node.as_fields();

        write![
            f,
            [
                import_token.format(),
                dot_token.format(),
                meta_token.format(),
            ]
        ]
    }

    fn needs_parentheses(&self, item: &JsImportMetaExpression) -> bool {
        item.needs_parentheses()
    }
}
