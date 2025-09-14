use crate::prelude::*;

use check_formatter::write;
use check_js_syntax::TsNonNullAssertionAssignment;
use check_js_syntax::TsNonNullAssertionAssignmentFields;
use check_js_syntax::parentheses::NeedsParentheses;

#[derive(Debug, Clone, Default)]
pub struct FormatTsNonNullAssertionAssignment;

impl FormatNodeRule<TsNonNullAssertionAssignment> for FormatTsNonNullAssertionAssignment {
    fn fmt_fields(
        &self,
        node: &TsNonNullAssertionAssignment,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsNonNullAssertionAssignmentFields {
            assignment,
            excl_token,
        } = node.as_fields();
        write![f, [assignment.format(), excl_token.format()]]
    }

    fn needs_parentheses(&self, item: &TsNonNullAssertionAssignment) -> bool {
        item.needs_parentheses()
    }
}
