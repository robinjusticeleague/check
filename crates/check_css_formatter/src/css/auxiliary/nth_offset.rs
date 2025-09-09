use crate::prelude::*;
use check_css_syntax::{CssNthOffset, CssNthOffsetFields};
use check_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssNthOffset;
impl FormatNodeRule<CssNthOffset> for FormatCssNthOffset {
    fn fmt_fields(&self, node: &CssNthOffset, f: &mut CssFormatter) -> FormatResult<()> {
        let CssNthOffsetFields { sign, value } = node.as_fields();

        write!(f, [sign.format(), space(), value.format()])
    }
}
