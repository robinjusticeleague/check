use crate::FormatBogusNodeRule;
use check_graphql_syntax::GraphqlBogusValue;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlBogusValue;
impl FormatBogusNodeRule<GraphqlBogusValue> for FormatGraphqlBogusValue {}
