use crate::parser::CssParser;
use crate::syntax::block::ParseBlockBody;
use crate::syntax::{DeclarationList, is_at_declaration};
use check_css_syntax::CssSyntaxKind;
use check_css_syntax::CssSyntaxKind::*;
use check_parser::CompletedMarker;
use check_parser::parse_lists::ParseNodeList;

#[inline]
pub(crate) fn parse_declaration_block(p: &mut CssParser) -> CompletedMarker {
    DeclarationBlock.parse_block_body(p)
}
struct DeclarationBlock;

impl ParseBlockBody for DeclarationBlock {
    const BLOCK_KIND: CssSyntaxKind = CSS_DECLARATION_BLOCK;

    fn is_at_element(&self, p: &mut CssParser) -> bool {
        is_at_declaration(p)
    }

    fn parse_list(&mut self, p: &mut CssParser) {
        DeclarationList.parse_list(p);
    }
}
