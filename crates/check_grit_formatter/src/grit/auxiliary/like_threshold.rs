use crate::prelude::*;
use check_formatter::write;
use check_grit_syntax::{GritLikeThreshold, GritLikeThresholdFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritLikeThreshold;
impl FormatNodeRule<GritLikeThreshold> for FormatGritLikeThreshold {
    fn fmt_fields(&self, node: &GritLikeThreshold, f: &mut GritFormatter) -> FormatResult<()> {
        let GritLikeThresholdFields {
            l_paren_token,
            threshold,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_paren_token.format(),
                threshold.format(),
                r_paren_token.format()
            ]
        )
    }
}
