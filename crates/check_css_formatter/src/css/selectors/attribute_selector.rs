use crate::prelude::*;
use check_css_syntax::{CssAttributeSelector, CssAttributeSelectorFields};
use check_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssAttributeSelector;
impl FormatNodeRule<CssAttributeSelector> for FormatCssAttributeSelector {
    fn fmt_fields(&self, node: &CssAttributeSelector, f: &mut CssFormatter) -> FormatResult<()> {
        let CssAttributeSelectorFields {
            l_brack_token,
            name,
            matcher,
            r_brack_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_brack_token.format(),
                name.format(),
                matcher.format(),
                r_brack_token.format()
            ]
        )
    }
}
