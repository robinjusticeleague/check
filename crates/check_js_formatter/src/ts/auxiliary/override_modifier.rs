use crate::prelude::*;

use check_formatter::write;
use check_js_syntax::TsOverrideModifier;
use check_js_syntax::TsOverrideModifierFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsOverrideModifier;

impl FormatNodeRule<TsOverrideModifier> for FormatTsOverrideModifier {
    fn fmt_fields(&self, node: &TsOverrideModifier, f: &mut JsFormatter) -> FormatResult<()> {
        let TsOverrideModifierFields { modifier_token } = node.as_fields();
        write![f, [modifier_token.format()]]
    }
}
