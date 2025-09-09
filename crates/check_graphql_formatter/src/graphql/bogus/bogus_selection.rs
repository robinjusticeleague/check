use crate::FormatBogusNodeRule;
use check_graphql_syntax::GraphqlBogusSelection;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlBogusSelection;
impl FormatBogusNodeRule<GraphqlBogusSelection> for FormatGraphqlBogusSelection {}
