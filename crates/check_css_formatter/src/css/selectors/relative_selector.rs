use crate::prelude::*;
use check_css_syntax::{CssRelativeSelector, CssRelativeSelectorFields};
use check_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssRelativeSelector;
impl FormatNodeRule<CssRelativeSelector> for FormatCssRelativeSelector {
    fn fmt_fields(&self, node: &CssRelativeSelector, f: &mut CssFormatter) -> FormatResult<()> {
        let CssRelativeSelectorFields {
            combinator,
            selector,
        } = node.as_fields();

        if combinator.is_some() {
            write!(f, [combinator.format(), space()])?;
        }

        write!(f, [selector.format()])
    }
}
