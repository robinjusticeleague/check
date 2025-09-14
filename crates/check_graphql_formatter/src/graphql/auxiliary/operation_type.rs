use crate::prelude::*;
use check_formatter::write;
use check_graphql_syntax::{GraphqlOperationType, GraphqlOperationTypeFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlOperationType;
impl FormatNodeRule<GraphqlOperationType> for FormatGraphqlOperationType {
    fn fmt_fields(
        &self,
        node: &GraphqlOperationType,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        let GraphqlOperationTypeFields { value_token } = node.as_fields();

        write![f, [value_token.format(),]]
    }
}
