#![deny(clippy::use_self)]

pub use crate::generated::CssSyntaxFactory;
use check_css_syntax::CssLanguage;
use check_rowan::TreeBuilder;

mod generated;

// Re-exported for tests
#[doc(hidden)]
pub use check_css_syntax as syntax;

pub type CssSyntaxTreeBuilder = TreeBuilder<'static, CssLanguage, CssSyntaxFactory>;

pub use generated::node_factory as make;
