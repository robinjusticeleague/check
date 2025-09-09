use crate::parser::CssParser;
use crate::syntax::{RuleList, is_at_rule_list_element};
use check_css_syntax::CssSyntaxKind::*;
use check_css_syntax::{CssSyntaxKind, T};
use check_parser::CompletedMarker;
use check_parser::parse_lists::ParseNodeList;

use crate::syntax::block::ParseBlockBody;

#[inline]
pub(crate) fn parse_rule_block(p: &mut CssParser) -> CompletedMarker {
    RuleBlock.parse_block_body(p)
}
struct RuleBlock;

impl ParseBlockBody for RuleBlock {
    const BLOCK_KIND: CssSyntaxKind = CSS_RULE_BLOCK;

    fn is_at_element(&self, p: &mut CssParser) -> bool {
        is_at_rule_list_element(p)
    }

    fn parse_list(&mut self, p: &mut CssParser) {
        RuleList::new(T!['}']).parse_list(p);
    }
}
