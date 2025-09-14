use crate::parser::CssParser;
use crate::syntax::block::parse_declaration_block;
use check_css_syntax::CssSyntaxKind::*;
use check_css_syntax::T;
use check_parser::parsed_syntax::ParsedSyntax::Present;
use check_parser::prelude::ParsedSyntax::Absent;
use check_parser::prelude::*;

#[inline]
pub(crate) fn is_at_font_face_at_rule(p: &mut CssParser) -> bool {
    p.at(T![font_face])
}

#[inline]
pub(crate) fn parse_font_face_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_font_face_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![font_face]);

    parse_declaration_block(p);

    Present(m.complete(p, CSS_FONT_FACE_AT_RULE))
}
