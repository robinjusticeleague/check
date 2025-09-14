use crate::prelude::*;
use check_css_syntax::{CssDocumentAtRule, CssDocumentAtRuleFields};
use check_formatter::write;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssDocumentAtRule;
impl FormatNodeRule<CssDocumentAtRule> for FormatCssDocumentAtRule {
    fn fmt_fields(&self, node: &CssDocumentAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let CssDocumentAtRuleFields {
            document_token,
            matchers,
            block,
        } = node.as_fields();

        write!(
            f,
            [
                document_token.format(),
                space(),
                matchers.format(),
                space(),
                block.format()
            ]
        )
    }
}
