//! This module defines the Concrete Syntax Tree used by Check.
//!
//! The tree is entirely lossless, whitespace, comments, and errors are preserved.
//! It also provides traversal methods including parent, children, and siblings of nodes.
//!
//! This is a simple wrapper around the `rowan` crate which does most of the heavy lifting and is language agnostic.

use crate::{GraphqlRoot, GraphqlSyntaxKind};
use check_rowan::Language;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct GraphqlLanguage;

impl Language for GraphqlLanguage {
    type Kind = GraphqlSyntaxKind;
    type Root = GraphqlRoot;
}

pub type GraphqlSyntaxNode = check_rowan::SyntaxNode<GraphqlLanguage>;
pub type GraphqlSyntaxToken = check_rowan::SyntaxToken<GraphqlLanguage>;
pub type GraphqlSyntaxElement = check_rowan::SyntaxElement<GraphqlLanguage>;
pub type GraphqlSyntaxElementChildren = check_rowan::SyntaxElementChildren<GraphqlLanguage>;
pub type GraphqlSyntaxList = check_rowan::SyntaxList<GraphqlLanguage>;
