use crate::prelude::*;
use check_formatter::write;
use check_grit_syntax::GritAnnotation;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritAnnotation;
impl FormatNodeRule<GritAnnotation> for FormatGritAnnotation {
    fn fmt_fields(&self, node: &GritAnnotation, f: &mut GritFormatter) -> FormatResult<()> {
        write!(f, [node.value_token().format()])
    }
}
