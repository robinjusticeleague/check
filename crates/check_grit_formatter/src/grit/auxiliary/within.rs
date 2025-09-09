use crate::prelude::*;
use check_formatter::write;
use check_grit_syntax::{GritWithin, GritWithinFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritWithin;
impl FormatNodeRule<GritWithin> for FormatGritWithin {
    fn fmt_fields(&self, node: &GritWithin, f: &mut GritFormatter) -> FormatResult<()> {
        let GritWithinFields {
            pattern,
            within_token,
            until_clause,
        } = node.as_fields();

        write!(
            f,
            [
                within_token.format(),
                space(),
                pattern.format(),
                space(),
                until_clause.format()
            ]
        )
    }
}
