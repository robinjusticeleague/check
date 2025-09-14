use crate::prelude::*;
use check_formatter::write;
use check_grit_syntax::{GritModOperation, GritModOperationFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritModOperation;
impl FormatNodeRule<GritModOperation> for FormatGritModOperation {
    fn fmt_fields(&self, node: &GritModOperation, f: &mut GritFormatter) -> FormatResult<()> {
        let GritModOperationFields {
            left,
            right,
            remainder_token,
        } = node.as_fields();

        write!(
            f,
            [
                left.format(),
                space(),
                remainder_token.format(),
                space(),
                right.format()
            ]
        )
    }
}
