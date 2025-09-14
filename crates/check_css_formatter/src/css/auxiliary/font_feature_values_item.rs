use crate::prelude::*;
use check_css_syntax::{CssFontFeatureValuesItem, CssFontFeatureValuesItemFields};
use check_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssFontFeatureValuesItem;
impl FormatNodeRule<CssFontFeatureValuesItem> for FormatCssFontFeatureValuesItem {
    fn fmt_fields(
        &self,
        node: &CssFontFeatureValuesItem,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssFontFeatureValuesItemFields {
            at_token,
            name,
            block,
        } = node.as_fields();

        write!(
            f,
            [at_token.format(), name.format(), space(), block.format()]
        )
    }
}
