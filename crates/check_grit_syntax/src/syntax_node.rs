//! This module defines the Concrete Syntax Tree for Grit used by Check.
//!
//! The tree is entirely lossless, whitespace, comments, and errors are preserved.
//! It also provides traversal methods including parent, children, and siblings of nodes.
//!
//! This is a simple wrapper around the `rowan` crate which does most of the heavy lifting and is language agnostic.

use crate::{GritRoot, GritSyntaxKind};
use check_rowan::Language;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct GritLanguage;

impl Language for GritLanguage {
    type Kind = GritSyntaxKind;
    type Root = GritRoot;
}

pub type GritSyntaxNode = check_rowan::SyntaxNode<GritLanguage>;
pub type GritSyntaxToken = check_rowan::SyntaxToken<GritLanguage>;
pub type GritSyntaxElement = check_rowan::SyntaxElement<GritLanguage>;
pub type GritSyntaxNodeChildren = check_rowan::SyntaxNodeChildren<GritLanguage>;
pub type GritSyntaxElementChildren = check_rowan::SyntaxElementChildren<GritLanguage>;
pub type GritSyntaxList = check_rowan::SyntaxList<GritLanguage>;
pub type GritSyntaxTrivia = check_rowan::syntax::SyntaxTrivia<GritLanguage>;
