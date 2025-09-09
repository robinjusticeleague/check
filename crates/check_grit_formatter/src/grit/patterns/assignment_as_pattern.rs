use crate::prelude::*;
use check_formatter::write;
use check_grit_syntax::{GritAssignmentAsPattern, GritAssignmentAsPatternFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritAssignmentAsPattern;
impl FormatNodeRule<GritAssignmentAsPattern> for FormatGritAssignmentAsPattern {
    fn fmt_fields(
        &self,
        node: &GritAssignmentAsPattern,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        let GritAssignmentAsPatternFields {
            pattern,
            eq_token,
            container,
        } = node.as_fields();

        write!(
            f,
            [
                container.format(),
                space(),
                eq_token.format(),
                space(),
                pattern.format()
            ]
        )
    }
}
