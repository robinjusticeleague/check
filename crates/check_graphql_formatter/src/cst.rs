use crate::prelude::*;
use check_formatter::{FormatOwnedWithRule, FormatRefWithRule, FormatResult};
use check_graphql_syntax::{GraphqlSyntaxNode, map_syntax_node};

#[derive(Debug, Copy, Clone, Default)]
pub struct FormatGraphqlSyntaxNode;

impl FormatRule<GraphqlSyntaxNode> for FormatGraphqlSyntaxNode {
    type Context = GraphqlFormatContext;

    fn fmt(&self, node: &GraphqlSyntaxNode, f: &mut GraphqlFormatter) -> FormatResult<()> {
        map_syntax_node!(node.clone(), node => node.format().fmt(f))
    }
}

impl AsFormat<GraphqlFormatContext> for GraphqlSyntaxNode {
    type Format<'a> = FormatRefWithRule<'a, Self, FormatGraphqlSyntaxNode>;

    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, FormatGraphqlSyntaxNode)
    }
}

impl IntoFormat<GraphqlFormatContext> for GraphqlSyntaxNode {
    type Format = FormatOwnedWithRule<Self, FormatGraphqlSyntaxNode>;

    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, FormatGraphqlSyntaxNode)
    }
}
