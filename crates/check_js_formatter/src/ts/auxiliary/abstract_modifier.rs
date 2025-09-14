use crate::prelude::*;

use check_formatter::write;
use check_js_syntax::TsAbstractModifier;
use check_js_syntax::TsAbstractModifierFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsAbstractModifier;

impl FormatNodeRule<TsAbstractModifier> for FormatTsAbstractModifier {
    fn fmt_fields(&self, node: &TsAbstractModifier, f: &mut JsFormatter) -> FormatResult<()> {
        let TsAbstractModifierFields { modifier_token } = node.as_fields();

        write![f, [modifier_token.format()]]
    }
}
