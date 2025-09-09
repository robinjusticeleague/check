use crate::prelude::*;

use check_formatter::write;
use check_js_syntax::{JsxName, JsxNameFields};

#[derive(Debug, Clone, Default)]
pub struct FormatJsxName;

impl FormatNodeRule<JsxName> for FormatJsxName {
    fn fmt_fields(&self, node: &JsxName, f: &mut JsFormatter) -> FormatResult<()> {
        let JsxNameFields { value_token } = node.as_fields();

        write![f, [value_token.format()]]
    }
}
