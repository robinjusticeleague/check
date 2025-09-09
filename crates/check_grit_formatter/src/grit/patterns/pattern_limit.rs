use crate::prelude::*;
use check_formatter::write;
use check_grit_syntax::{GritPatternLimit, GritPatternLimitFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPatternLimit;
impl FormatNodeRule<GritPatternLimit> for FormatGritPatternLimit {
    fn fmt_fields(&self, node: &GritPatternLimit, f: &mut GritFormatter) -> FormatResult<()> {
        let GritPatternLimitFields {
            limit_token,
            limit,
            pattern,
        } = node.as_fields();

        write!(
            f,
            [
                pattern.format(),
                space(),
                limit_token.format(),
                space(),
                limit.format()
            ]
        )
    }
}
