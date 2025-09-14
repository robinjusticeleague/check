use crate::prelude::*;
use check_formatter::write;
use check_grit_syntax::{GritEvery, GritEveryFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritEvery;
impl FormatNodeRule<GritEvery> for FormatGritEvery {
    fn fmt_fields(&self, node: &GritEvery, f: &mut GritFormatter) -> FormatResult<()> {
        let GritEveryFields {
            pattern,
            every_token,
        } = node.as_fields();

        write!(f, [every_token.format(), space(), pattern.format()])
    }
}
