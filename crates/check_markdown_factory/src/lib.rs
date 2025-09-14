#![deny(clippy::use_self)]

use check_markdown_syntax::MarkdownLanguage;
use check_rowan::TreeBuilder;

mod generated;
pub use crate::generated::MarkdownSyntaxFactory;

// Re-exported for tests
#[doc(hidden)]
pub use check_markdown_syntax as syntax;

pub type DemoSyntaxTreeBuilder = TreeBuilder<'static, MarkdownLanguage, MarkdownSyntaxFactory>;

pub mod make;
