use crate::prelude::*;
use check_css_syntax::{CssImportAnonymousLayer, CssImportAnonymousLayerFields};
use check_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssImportAnonymousLayer;
impl FormatNodeRule<CssImportAnonymousLayer> for FormatCssImportAnonymousLayer {
    fn fmt_fields(&self, node: &CssImportAnonymousLayer, f: &mut CssFormatter) -> FormatResult<()> {
        let CssImportAnonymousLayerFields { layer_token } = node.as_fields();

        write!(f, [layer_token.format()])
    }
}
