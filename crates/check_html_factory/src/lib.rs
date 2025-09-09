#![deny(clippy::use_self)]

use check_html_syntax::HtmlLanguage;
use check_rowan::TreeBuilder;

mod generated;
pub use crate::generated::HtmlSyntaxFactory;
pub mod make;

// Re-exported for tests
#[doc(hidden)]
pub use check_html_syntax as syntax;

pub type JsonSyntaxTreeBuilder = TreeBuilder<'static, HtmlLanguage, HtmlSyntaxFactory>;
