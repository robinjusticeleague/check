use crate::prelude::*;
use check_formatter::write;
use check_grit_syntax::{GritPredicateReturn, GritPredicateReturnFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateReturn;
impl FormatNodeRule<GritPredicateReturn> for FormatGritPredicateReturn {
    fn fmt_fields(&self, node: &GritPredicateReturn, f: &mut GritFormatter) -> FormatResult<()> {
        let GritPredicateReturnFields {
            pattern,
            return_token,
        } = node.as_fields();

        write!(f, [return_token.format(), space(), pattern.format()])
    }
}
