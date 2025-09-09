use crate::prelude::*;
use check_css_syntax::{CssPseudoClassNthIdentifier, CssPseudoClassNthIdentifierFields};
use check_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoClassNthIdentifier;
impl FormatNodeRule<CssPseudoClassNthIdentifier> for FormatCssPseudoClassNthIdentifier {
    fn fmt_fields(
        &self,
        node: &CssPseudoClassNthIdentifier,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssPseudoClassNthIdentifierFields { value } = node.as_fields();

        write!(f, [value.format()])
    }
}
