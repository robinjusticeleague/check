use crate::prelude::*;
use check_formatter::write;
use check_grit_syntax::GritDot;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritDot;
impl FormatNodeRule<GritDot> for FormatGritDot {
    fn fmt_fields(&self, node: &GritDot, f: &mut GritFormatter) -> FormatResult<()> {
        write!(f, [node.dot_token().format()])
    }
}
