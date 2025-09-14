use crate::prelude::*;
use check_formatter::write;

use check_js_syntax::TsGlobalDeclaration;
use check_js_syntax::TsGlobalDeclarationFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsGlobalDeclaration;

impl FormatNodeRule<TsGlobalDeclaration> for FormatTsGlobalDeclaration {
    fn fmt_fields(&self, node: &TsGlobalDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        let TsGlobalDeclarationFields { global_token, body } = node.as_fields();

        write![f, [global_token.format(), space(), body.format()]]
    }
}
