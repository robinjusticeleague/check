use crate::prelude::*;
use check_formatter::write;
use check_graphql_syntax::{GraphqlArgument, GraphqlArgumentFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlArgument;
impl FormatNodeRule<GraphqlArgument> for FormatGraphqlArgument {
    fn fmt_fields(&self, node: &GraphqlArgument, f: &mut GraphqlFormatter) -> FormatResult<()> {
        let GraphqlArgumentFields {
            name,
            colon_token,
            value,
        } = node.as_fields();

        write!(
            f,
            [name.format(), colon_token.format(), space(), value.format(),]
        )
    }
}
