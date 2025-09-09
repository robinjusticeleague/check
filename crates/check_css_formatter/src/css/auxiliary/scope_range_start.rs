use crate::prelude::*;
use check_css_syntax::{CssScopeRangeStart, CssScopeRangeStartFields};
use check_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssScopeRangeStart;
impl FormatNodeRule<CssScopeRangeStart> for FormatCssScopeRangeStart {
    fn fmt_fields(&self, node: &CssScopeRangeStart, f: &mut CssFormatter) -> FormatResult<()> {
        let CssScopeRangeStartFields { start } = node.as_fields();

        write!(f, [start.format()])
    }
}
