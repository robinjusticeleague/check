use crate::prelude::*;
use check_formatter::write;
use check_graphql_syntax::{GraphqlBooleanValue, GraphqlBooleanValueFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlBooleanValue;
impl FormatNodeRule<GraphqlBooleanValue> for FormatGraphqlBooleanValue {
    fn fmt_fields(&self, node: &GraphqlBooleanValue, f: &mut GraphqlFormatter) -> FormatResult<()> {
        let GraphqlBooleanValueFields { value_token } = node.as_fields();
        write![f, [value_token.format()]]
    }
}
