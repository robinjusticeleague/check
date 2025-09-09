pub use crate::generated::node_factory::*;
use check_graphql_syntax::{GraphqlSyntaxKind, GraphqlSyntaxToken};

/// Create a new literal name token with no attached trivia
pub fn ident(text: &str) -> GraphqlSyntaxToken {
    GraphqlSyntaxToken::new_detached(GraphqlSyntaxKind::IDENT, text, [], [])
}
