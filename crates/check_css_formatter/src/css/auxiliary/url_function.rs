use crate::prelude::*;
use check_css_syntax::{CssUrlFunction, CssUrlFunctionFields};
use check_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssUrlFunction;
impl FormatNodeRule<CssUrlFunction> for FormatCssUrlFunction {
    fn fmt_fields(&self, node: &CssUrlFunction, f: &mut CssFormatter) -> FormatResult<()> {
        let CssUrlFunctionFields {
            name,
            l_paren_token,
            value,
            modifiers,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                name.format(),
                l_paren_token.format(),
                value.format(),
                modifiers.format(),
                r_paren_token.format()
            ]
        )
    }
}
