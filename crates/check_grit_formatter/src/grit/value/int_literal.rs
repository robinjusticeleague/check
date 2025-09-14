use crate::prelude::*;
use check_formatter::write;
use check_grit_syntax::{GritIntLiteral, GritIntLiteralFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritIntLiteral;
impl FormatNodeRule<GritIntLiteral> for FormatGritIntLiteral {
    fn fmt_fields(&self, node: &GritIntLiteral, f: &mut GritFormatter) -> FormatResult<()> {
        let GritIntLiteralFields { value_token } = node.as_fields();
        write!(f, [value_token.format()])
    }
}
