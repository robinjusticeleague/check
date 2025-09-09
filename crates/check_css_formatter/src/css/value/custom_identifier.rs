use crate::prelude::*;
use check_css_syntax::{CssCustomIdentifier, CssCustomIdentifierFields};
use check_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssCustomIdentifier;
impl FormatNodeRule<CssCustomIdentifier> for FormatCssCustomIdentifier {
    fn fmt_fields(&self, node: &CssCustomIdentifier, f: &mut CssFormatter) -> FormatResult<()> {
        let CssCustomIdentifierFields { value_token } = node.as_fields();

        write!(f, [value_token.format()])
    }
}
