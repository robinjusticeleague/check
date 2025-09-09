use crate::prelude::*;
use check_formatter::write;
use check_grit_syntax::GritName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritName;
impl FormatNodeRule<GritName> for FormatGritName {
    fn fmt_fields(&self, node: &GritName, f: &mut GritFormatter) -> FormatResult<()> {
        write!(f, [node.value_token().format()])
    }
}
