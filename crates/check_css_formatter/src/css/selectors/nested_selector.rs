use crate::prelude::*;
use check_css_syntax::{CssNestedSelector, CssNestedSelectorFields};
use check_formatter::write;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssNestedSelector;
impl FormatNodeRule<CssNestedSelector> for FormatCssNestedSelector {
    fn fmt_fields(&self, node: &CssNestedSelector, f: &mut CssFormatter) -> FormatResult<()> {
        let CssNestedSelectorFields { amp_token } = node.as_fields();
        write!(f, [amp_token.format()])
    }
}
