use crate::prelude::*;

use check_formatter::write;
use check_js_syntax::TsQualifiedModuleName;
use check_js_syntax::TsQualifiedModuleNameFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsQualifiedModuleName;

impl FormatNodeRule<TsQualifiedModuleName> for FormatTsQualifiedModuleName {
    fn fmt_fields(&self, node: &TsQualifiedModuleName, f: &mut JsFormatter) -> FormatResult<()> {
        let TsQualifiedModuleNameFields {
            left,
            dot_token,
            right,
        } = node.as_fields();

        write![f, [left.format(), dot_token.format(), right.format(),]]
    }
}
