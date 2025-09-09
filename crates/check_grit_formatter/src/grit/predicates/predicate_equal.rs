use crate::prelude::*;
use check_formatter::write;
use check_grit_syntax::{GritPredicateEqual, GritPredicateEqualFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateEqual;
impl FormatNodeRule<GritPredicateEqual> for FormatGritPredicateEqual {
    fn fmt_fields(&self, node: &GritPredicateEqual, f: &mut GritFormatter) -> FormatResult<()> {
        let GritPredicateEqualFields {
            right,
            left,
            equality_token,
        } = node.as_fields();

        write!(
            f,
            [
                left.format(),
                space(),
                equality_token.format(),
                space(),
                right.format()
            ]
        )
    }
}
