use crate::prelude::*;
use check_formatter::write;
use check_grit_syntax::{GritBooleanLiteral, GritBooleanLiteralFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritBooleanLiteral;
impl FormatNodeRule<GritBooleanLiteral> for FormatGritBooleanLiteral {
    fn fmt_fields(&self, node: &GritBooleanLiteral, f: &mut GritFormatter) -> FormatResult<()> {
        let GritBooleanLiteralFields { value } = node.as_fields();
        write!(f, [value.format()])
    }
}
