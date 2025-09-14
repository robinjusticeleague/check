use crate::prelude::*;
use crate::utils::{FormatLiteralStringToken, StringLiteralParentKind};

use check_formatter::write;
use check_js_syntax::JsxString;

#[derive(Debug, Clone, Default)]
pub struct FormatJsxString;

impl FormatNodeRule<JsxString> for FormatJsxString {
    fn fmt_fields(&self, node: &JsxString, f: &mut JsFormatter) -> FormatResult<()> {
        write![
            f,
            [FormatLiteralStringToken::new(
                &node.value_token()?,
                StringLiteralParentKind::Expression
            )]
        ]
    }
}
