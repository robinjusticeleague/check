use crate::prelude::*;
use check_formatter::write;
use check_graphql_syntax::{GraphqlDirective, GraphqlDirectiveFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlDirective;
impl FormatNodeRule<GraphqlDirective> for FormatGraphqlDirective {
    fn fmt_fields(&self, node: &GraphqlDirective, f: &mut GraphqlFormatter) -> FormatResult<()> {
        let GraphqlDirectiveFields {
            at_token,
            name,
            arguments,
        } = node.as_fields();

        write![f, [at_token.format(), name.format(), arguments.format()]]
    }
}
