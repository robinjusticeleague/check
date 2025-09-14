use crate::prelude::*;
use check_css_syntax::{CssPseudoElementIdentifier, CssPseudoElementIdentifierFields};
use check_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoElementIdentifier;
impl FormatNodeRule<CssPseudoElementIdentifier> for FormatCssPseudoElementIdentifier {
    fn fmt_fields(
        &self,
        node: &CssPseudoElementIdentifier,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssPseudoElementIdentifierFields { name } = node.as_fields();

        write!(f, [name.format()])
    }
}
