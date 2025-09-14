use crate::prelude::*;

use check_formatter::write;
use check_js_syntax::JsNullLiteralExpression;
use check_js_syntax::JsNullLiteralExpressionFields;
use check_js_syntax::parentheses::NeedsParentheses;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsNullLiteralExpression;

impl FormatNodeRule<JsNullLiteralExpression> for FormatJsNullLiteralExpression {
    fn fmt_fields(&self, node: &JsNullLiteralExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsNullLiteralExpressionFields { value_token } = node.as_fields();

        write![f, [value_token.format()]]
    }

    fn needs_parentheses(&self, item: &JsNullLiteralExpression) -> bool {
        item.needs_parentheses()
    }
}
