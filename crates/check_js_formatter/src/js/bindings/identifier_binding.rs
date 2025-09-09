use crate::prelude::*;

use check_formatter::write;
use check_js_syntax::JsIdentifierBinding;
use check_js_syntax::JsIdentifierBindingFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsIdentifierBinding;

impl FormatNodeRule<JsIdentifierBinding> for FormatJsIdentifierBinding {
    fn fmt_fields(&self, node: &JsIdentifierBinding, f: &mut JsFormatter) -> FormatResult<()> {
        let JsIdentifierBindingFields { name_token } = node.as_fields();

        write![f, [name_token.format()]]
    }
}
