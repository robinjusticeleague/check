//! This module defines the Concrete Syntax Tree used by Check.
//!
//! The tree is entirely lossless, whitespace, comments, and errors are preserved.
//! It also provides traversal methods including parent, children, and siblings of nodes.
//!
//! This is a simple wrapper around the `rowan` crate which does most of the heavy lifting and is language agnostic.

use crate::{JsonRoot, JsonSyntaxKind};
use check_rowan::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct JsonLanguage;

impl Language for JsonLanguage {
    type Kind = JsonSyntaxKind;
    type Root = JsonRoot;
}

pub type JsonSyntaxNode = check_rowan::SyntaxNode<JsonLanguage>;
pub type JsonSyntaxToken = check_rowan::SyntaxToken<JsonLanguage>;
pub type JsonSyntaxElement = check_rowan::SyntaxElement<JsonLanguage>;
pub type JsonSyntaxNodeChildren = check_rowan::SyntaxNodeChildren<JsonLanguage>;
pub type JsonSyntaxElementChildren = check_rowan::SyntaxElementChildren<JsonLanguage>;
pub type JsonSyntaxList = check_rowan::SyntaxList<JsonLanguage>;
