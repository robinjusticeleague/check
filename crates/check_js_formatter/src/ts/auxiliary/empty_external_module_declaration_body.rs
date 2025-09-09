use crate::prelude::*;

use crate::utils::FormatOptionalSemicolon;
use check_formatter::write;
use check_js_syntax::TsEmptyExternalModuleDeclarationBody;
use check_js_syntax::TsEmptyExternalModuleDeclarationBodyFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsEmptyExternalModuleDeclarationBody;

impl FormatNodeRule<TsEmptyExternalModuleDeclarationBody>
    for FormatTsEmptyExternalModuleDeclarationBody
{
    fn fmt_fields(
        &self,
        node: &TsEmptyExternalModuleDeclarationBody,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsEmptyExternalModuleDeclarationBodyFields { semicolon_token } = node.as_fields();
        write![f, [FormatOptionalSemicolon::new(Some(&semicolon_token?))]]
    }
}
