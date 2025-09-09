use crate::prelude::*;
use check_formatter::write;
use check_grit_syntax::{GritPatternUntilClause, GritPatternUntilClauseFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPatternUntilClause;
impl FormatNodeRule<GritPatternUntilClause> for FormatGritPatternUntilClause {
    fn fmt_fields(&self, node: &GritPatternUntilClause, f: &mut GritFormatter) -> FormatResult<()> {
        let GritPatternUntilClauseFields { until, until_token } = node.as_fields();

        write!(f, [until_token.format(), space(), until.format()])
    }
}
