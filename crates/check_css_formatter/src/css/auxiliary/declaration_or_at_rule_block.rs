use crate::prelude::*;
use crate::utils::block_like::FormatCssBlockLike;
use check_css_syntax::CssDeclarationOrAtRuleBlock;
use check_css_syntax::stmt_ext::CssBlockLike;
use check_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssDeclarationOrAtRuleBlock;
impl FormatNodeRule<CssDeclarationOrAtRuleBlock> for FormatCssDeclarationOrAtRuleBlock {
    fn fmt_fields(
        &self,
        node: &CssDeclarationOrAtRuleBlock,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        write!(
            f,
            [FormatCssBlockLike::new(&CssBlockLike::from(node.clone()))]
        )
    }

    fn fmt_dangling_comments(
        &self,
        _: &CssDeclarationOrAtRuleBlock,
        _: &mut CssFormatter,
    ) -> FormatResult<()> {
        // Formatted inside of `fmt_fields`
        Ok(())
    }
}
