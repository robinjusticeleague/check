use crate::prelude::*;
use check_formatter::write;
use check_grit_syntax::{GritPatternIncludes, GritPatternIncludesFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPatternIncludes;
impl FormatNodeRule<GritPatternIncludes> for FormatGritPatternIncludes {
    fn fmt_fields(&self, node: &GritPatternIncludes, f: &mut GritFormatter) -> FormatResult<()> {
        let GritPatternIncludesFields {
            includes,
            includes_token,
        } = node.as_fields();

        write!(f, [includes_token.format(), space(), includes.format()])
    }
}
