use crate::prelude::*;
use check_css_syntax::{CssKeyframesItem, CssKeyframesItemFields};
use check_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssKeyframesItem;
impl FormatNodeRule<CssKeyframesItem> for FormatCssKeyframesItem {
    fn fmt_fields(&self, node: &CssKeyframesItem, f: &mut CssFormatter) -> FormatResult<()> {
        let CssKeyframesItemFields { selectors, block } = node.as_fields();

        write!(
            f,
            [
                group(&selectors.format()).should_expand(true),
                space(),
                block.format()
            ]
        )
    }
}
