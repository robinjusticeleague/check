use crate::prelude::*;
use check_formatter::write;
use check_grit_syntax::{GritBracketedPattern, GritBracketedPatternFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritBracketedPattern;
impl FormatNodeRule<GritBracketedPattern> for FormatGritBracketedPattern {
    fn fmt_fields(&self, node: &GritBracketedPattern, f: &mut GritFormatter) -> FormatResult<()> {
        let GritBracketedPatternFields {
            l_paren_token,
            r_paren_token,
            pattern,
        } = node.as_fields();

        write!(
            f,
            [
                l_paren_token.format(),
                pattern.format(),
                r_paren_token.format()
            ]
        )
    }
}
