use crate::prelude::*;

use check_formatter::write;
use check_js_syntax::TsParenthesizedType;
use check_js_syntax::TsParenthesizedTypeFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsParenthesizedType;

impl FormatNodeRule<TsParenthesizedType> for FormatTsParenthesizedType {
    fn fmt_fields(&self, node: &TsParenthesizedType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsParenthesizedTypeFields {
            l_paren_token,
            ty,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [l_paren_token.format(), &ty.format(), r_paren_token.format()]
        )
    }
}
