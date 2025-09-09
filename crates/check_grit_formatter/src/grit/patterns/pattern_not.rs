use crate::prelude::*;
use check_formatter::write;
use check_grit_syntax::{GritPatternNot, GritPatternNotFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPatternNot;
impl FormatNodeRule<GritPatternNot> for FormatGritPatternNot {
    fn fmt_fields(&self, node: &GritPatternNot, f: &mut GritFormatter) -> FormatResult<()> {
        let GritPatternNotFields { pattern, not } = node.as_fields();

        write!(f, [not.format(), pattern.format()])
    }
}
