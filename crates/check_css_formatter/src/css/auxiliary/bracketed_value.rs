use crate::prelude::*;
use check_css_syntax::{CssBracketedValue, CssBracketedValueFields};
use check_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBracketedValue;
impl FormatNodeRule<CssBracketedValue> for FormatCssBracketedValue {
    fn fmt_fields(&self, node: &CssBracketedValue, f: &mut CssFormatter) -> FormatResult<()> {
        let CssBracketedValueFields {
            l_brack_token,
            items,
            r_brack_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_brack_token.format(),
                items.format(),
                r_brack_token.format()
            ]
        )
    }
}
