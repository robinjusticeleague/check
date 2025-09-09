use crate::prelude::*;
use check_formatter::write;
use check_grit_syntax::{GritPatternWhere, GritPatternWhereFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPatternWhere;
impl FormatNodeRule<GritPatternWhere> for FormatGritPatternWhere {
    fn fmt_fields(&self, node: &GritPatternWhere, f: &mut GritFormatter) -> FormatResult<()> {
        let GritPatternWhereFields {
            pattern,
            side_condition,
            where_token,
        } = node.as_fields();

        write!(
            f,
            [
                space(),
                pattern.format(),
                space(),
                where_token.format(),
                space(),
                side_condition.format(),
            ]
        )
    }
}
