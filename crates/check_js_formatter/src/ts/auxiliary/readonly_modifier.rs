use crate::prelude::*;

use check_formatter::write;
use check_js_syntax::TsReadonlyModifier;
use check_js_syntax::TsReadonlyModifierFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsReadonlyModifier;

impl FormatNodeRule<TsReadonlyModifier> for FormatTsReadonlyModifier {
    fn fmt_fields(&self, node: &TsReadonlyModifier, f: &mut JsFormatter) -> FormatResult<()> {
        let TsReadonlyModifierFields { modifier_token } = node.as_fields();
        write![f, [modifier_token.format()]]
    }
}
