use crate::prelude::*;
use check_css_syntax::{CssValueAtRuleDeclarationClause, CssValueAtRuleDeclarationClauseFields};
use check_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssValueAtRuleDeclarationClause;
impl FormatNodeRule<CssValueAtRuleDeclarationClause> for FormatCssValueAtRuleDeclarationClause {
    fn fmt_fields(
        &self,
        node: &CssValueAtRuleDeclarationClause,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssValueAtRuleDeclarationClauseFields { properties } = node.as_fields();

        write!(f, [properties.format()])
    }
}
