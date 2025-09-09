use crate::prelude::*;

use check_formatter::write;
use check_js_syntax::{TsSymbolType, TsSymbolTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsSymbolType;

impl FormatNodeRule<TsSymbolType> for FormatTsSymbolType {
    fn fmt_fields(&self, node: &TsSymbolType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsSymbolTypeFields { symbol_token } = node.as_fields();

        write![f, [symbol_token.format()]]
    }
}
