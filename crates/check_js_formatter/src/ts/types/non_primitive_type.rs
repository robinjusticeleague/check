use crate::prelude::*;

use check_formatter::write;
use check_js_syntax::{TsNonPrimitiveType, TsNonPrimitiveTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsNonPrimitiveType;

impl FormatNodeRule<TsNonPrimitiveType> for FormatTsNonPrimitiveType {
    fn fmt_fields(&self, node: &TsNonPrimitiveType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsNonPrimitiveTypeFields { object_token } = node.as_fields();

        write![f, [object_token.format()]]
    }
}
