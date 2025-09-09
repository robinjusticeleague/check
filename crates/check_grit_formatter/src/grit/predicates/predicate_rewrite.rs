use crate::prelude::*;
use check_formatter::write;
use check_grit_syntax::{GritPredicateRewrite, GritPredicateRewriteFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateRewrite;
impl FormatNodeRule<GritPredicateRewrite> for FormatGritPredicateRewrite {
    fn fmt_fields(&self, node: &GritPredicateRewrite, f: &mut GritFormatter) -> FormatResult<()> {
        let GritPredicateRewriteFields {
            annotation,
            left,
            fat_arrow_token,
            right,
        } = node.as_fields();

        write!(
            f,
            [
                annotation.format(),
                space(),
                left.format(),
                space(),
                fat_arrow_token.format(),
                space(),
                right.format()
            ]
        )
    }
}
