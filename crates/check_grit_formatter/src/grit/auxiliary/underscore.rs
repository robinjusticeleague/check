use crate::prelude::*;
use check_formatter::write;
use check_grit_syntax::GritUnderscore;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritUnderscore;
impl FormatNodeRule<GritUnderscore> for FormatGritUnderscore {
    fn fmt_fields(&self, node: &GritUnderscore, f: &mut GritFormatter) -> FormatResult<()> {
        write!(f, [node.token_token().format()])
    }
}
