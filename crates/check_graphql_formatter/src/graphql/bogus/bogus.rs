use crate::FormatBogusNodeRule;
use check_graphql_syntax::GraphqlBogus;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlBogus;
impl FormatBogusNodeRule<GraphqlBogus> for FormatGraphqlBogus {}
