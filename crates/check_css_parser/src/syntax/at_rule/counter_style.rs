use crate::lexer::CssLexContext;
use crate::parser::CssParser;
use crate::syntax::block::parse_declaration_block;
use crate::syntax::parse_custom_identifier;
use crate::syntax::parse_error::expected_non_css_wide_keyword_identifier;
use check_css_syntax::CssSyntaxKind::*;
use check_css_syntax::{CssSyntaxKind, T};
use check_parser::parse_recovery::ParseRecoveryTokenSet;
use check_parser::parsed_syntax::ParsedSyntax::Present;
use check_parser::prelude::ParsedSyntax::Absent;
use check_parser::prelude::*;

#[inline]
pub(crate) fn is_at_counter_style_at_rule(p: &mut CssParser) -> bool {
    p.at(T![counter_style])
}

#[inline]
pub(crate) fn parse_counter_style_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_counter_style_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![counter_style]);

    let kind = if parse_custom_identifier(p, CssLexContext::Regular)
        .or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(CSS_BOGUS, COUNTER_STYLE_RECOVERY_SET)
                .enable_recovery_on_line_break(),
            expected_non_css_wide_keyword_identifier,
        )
        .is_ok()
    {
        CSS_COUNTER_STYLE_AT_RULE
    } else {
        CSS_BOGUS_AT_RULE
    };

    parse_declaration_block(p);

    Present(m.complete(p, kind))
}

const COUNTER_STYLE_RECOVERY_SET: TokenSet<CssSyntaxKind> = token_set![T!['{']];
