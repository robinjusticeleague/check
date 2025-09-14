#![deny(clippy::use_self)]

use check_json_syntax::JsonLanguage;
use check_rowan::TreeBuilder;

mod generated;
pub use crate::generated::JsonSyntaxFactory;
pub mod make;

// Re-exported for tests
#[doc(hidden)]
pub use check_json_syntax as syntax;

pub type JsonSyntaxTreeBuilder = TreeBuilder<'static, JsonLanguage, JsonSyntaxFactory>;
