//! This module defines the Concrete Syntax Tree used by Check.
//!
//! The tree is entirely lossless, whitespace, comments, and errors are preserved.
//! It also provides traversal methods including parent, children, and siblings of nodes.
//!
//! This is a simple wrapper around the `rowan` crate which does most of the heavy lifting and is language agnostic.

use crate::{AnyJsRoot, JsSyntaxKind};
use check_rowan::Language;
#[cfg(feature = "schema")]
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "schema", derive(Serialize, schemars::JsonSchema))]
pub struct JsLanguage;

impl Language for JsLanguage {
    type Kind = JsSyntaxKind;
    type Root = AnyJsRoot;
}

pub type JsSyntaxNode = check_rowan::SyntaxNode<JsLanguage>;
pub type JsSyntaxToken = check_rowan::SyntaxToken<JsLanguage>;
pub type JsSyntaxElement = check_rowan::SyntaxElement<JsLanguage>;
pub type JsSyntaxNodeChildren = check_rowan::SyntaxNodeChildren<JsLanguage>;
pub type JsSyntaxElementChildren = check_rowan::SyntaxElementChildren<JsLanguage>;
pub type JsSyntaxList = check_rowan::SyntaxList<JsLanguage>;
pub type JsSyntaxTrivia = check_rowan::syntax::SyntaxTrivia<JsLanguage>;
