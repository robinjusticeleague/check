//! This module defines the Concrete Syntax Tree used by Check.
//!
//! The tree is entirely lossless, whitespace, comments, and errors are preserved.
//! It also provides traversal methods including parent, children, and siblings of nodes.
//!
//! This is a simple wrapper around the `rowan` crate which does most of the heavy lifting and is language agnostic.

use crate::{MarkdownSyntaxKind, MdDocument};
use check_rowan::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct MarkdownLanguage;

impl Language for MarkdownLanguage {
    type Kind = MarkdownSyntaxKind;
    type Root = MdDocument;
}

pub type MarkdownSyntaxNode = check_rowan::SyntaxNode<MarkdownLanguage>;
pub type MarkdownSyntaxToken = check_rowan::SyntaxToken<MarkdownLanguage>;
pub type MarkdownSyntaxElement = check_rowan::SyntaxElement<MarkdownLanguage>;
pub type MarkdownSyntaxNodeChildren = check_rowan::SyntaxNodeChildren<MarkdownLanguage>;
pub type MarkdownSyntaxElementChildren = check_rowan::SyntaxElementChildren<MarkdownLanguage>;
pub type MarkdownSyntaxList = check_rowan::SyntaxList<MarkdownLanguage>;
