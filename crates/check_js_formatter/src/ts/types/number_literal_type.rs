use crate::prelude::*;

use check_formatter::token::number::NumberFormatOptions;
use check_formatter::write;
use check_js_syntax::{TsNumberLiteralType, TsNumberLiteralTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsNumberLiteralType;

impl FormatNodeRule<TsNumberLiteralType> for FormatTsNumberLiteralType {
    fn fmt_fields(&self, node: &TsNumberLiteralType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsNumberLiteralTypeFields {
            minus_token,
            literal_token,
        } = node.as_fields();
        write![
            f,
            [
                minus_token.format(),
                format_number_token(
                    &literal_token?,
                    NumberFormatOptions::default().keep_one_trailing_decimal_zero()
                )
            ]
        ]
    }
}
