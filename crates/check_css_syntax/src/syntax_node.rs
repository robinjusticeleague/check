//! This module defines the Concrete Syntax Tree used by Check.
//!
//! The tree is entirely lossless, whitespace, comments, and errors are preserved.
//! It also provides traversal methods including parent, children, and siblings of nodes.
//!
//! This is a simple wrapper around the `rowan` crate which does most of the heavy lifting and is language agnostic.

use crate::{CssRoot, CssSyntaxKind};
use check_rowan::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CssLanguage;

impl Language for CssLanguage {
    type Kind = CssSyntaxKind;
    type Root = CssRoot;
}

pub type CssSyntaxNode = check_rowan::SyntaxNode<CssLanguage>;
pub type CssSyntaxToken = check_rowan::SyntaxToken<CssLanguage>;
pub type CssSyntaxElement = check_rowan::SyntaxElement<CssLanguage>;
pub type CssSyntaxNodeChildren = check_rowan::SyntaxNodeChildren<CssLanguage>;
pub type CssSyntaxElementChildren = check_rowan::SyntaxElementChildren<CssLanguage>;
pub type CssSyntaxList = check_rowan::SyntaxList<CssLanguage>;
