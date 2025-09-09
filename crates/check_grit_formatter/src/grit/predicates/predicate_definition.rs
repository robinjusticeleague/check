use crate::prelude::*;
use check_formatter::write;
use check_grit_syntax::{GritPredicateDefinition, GritPredicateDefinitionFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateDefinition;
impl FormatNodeRule<GritPredicateDefinition> for FormatGritPredicateDefinition {
    fn fmt_fields(
        &self,
        node: &GritPredicateDefinition,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        let GritPredicateDefinitionFields {
            l_paren_token,
            r_paren_token,
            args,
            body,
            name,
            predicate_token,
        } = node.as_fields();
        write!(
            f,
            [
                predicate_token.format(),
                space(),
                name.format(),
                l_paren_token.format(),
                group(&args.format()),
                r_paren_token.format(),
                space(),
                body.format()
            ]
        )
    }
}
