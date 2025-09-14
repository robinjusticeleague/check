use crate::prelude::*;
use check_formatter::write;
use check_grit_syntax::{GritJavascriptBodyWrapper, GritJavascriptBodyWrapperFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritJavascriptBodyWrapper;
impl FormatNodeRule<GritJavascriptBodyWrapper> for FormatGritJavascriptBodyWrapper {
    fn fmt_fields(
        &self,
        node: &GritJavascriptBodyWrapper,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        let GritJavascriptBodyWrapperFields { value_token } = node.as_fields();

        write!(f, [value_token.format()])
    }
}
