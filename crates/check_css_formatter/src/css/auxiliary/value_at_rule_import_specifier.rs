use crate::prelude::*;
use check_css_syntax::{CssValueAtRuleImportSpecifier, CssValueAtRuleImportSpecifierFields};
use check_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssValueAtRuleImportSpecifier;
impl FormatNodeRule<CssValueAtRuleImportSpecifier> for FormatCssValueAtRuleImportSpecifier {
    fn fmt_fields(
        &self,
        node: &CssValueAtRuleImportSpecifier,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssValueAtRuleImportSpecifierFields { name } = node.as_fields();

        write!(f, [name.format()])
    }
}
