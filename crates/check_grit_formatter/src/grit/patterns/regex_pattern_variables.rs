use crate::prelude::*;
use check_formatter::write;
use check_grit_syntax::{GritRegexPatternVariables, GritRegexPatternVariablesFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritRegexPatternVariables;
impl FormatNodeRule<GritRegexPatternVariables> for FormatGritRegexPatternVariables {
    fn fmt_fields(
        &self,
        node: &GritRegexPatternVariables,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        let GritRegexPatternVariablesFields {
            r_paren_token,
            l_paren_token,
            args,
        } = node.as_fields();

        write!(
            f,
            [
                l_paren_token.format(),
                args.format(),
                r_paren_token.format()
            ]
        )
    }
}
