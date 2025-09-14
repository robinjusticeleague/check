use crate::prelude::*;
use check_formatter::write;
use check_graphql_syntax::{GraphqlDirectiveLocation, GraphqlDirectiveLocationFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlDirectiveLocation;
impl FormatNodeRule<GraphqlDirectiveLocation> for FormatGraphqlDirectiveLocation {
    fn fmt_fields(
        &self,
        node: &GraphqlDirectiveLocation,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        let GraphqlDirectiveLocationFields { value_token } = node.as_fields();

        write!(f, [value_token.format()])
    }
}
