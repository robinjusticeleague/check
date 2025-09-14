#![deny(clippy::use_self)]

use check_rowan::TreeBuilder;
use check_tailwind_syntax::TailwindLanguage;

mod generated;
pub use crate::generated::TailwindSyntaxFactory;
pub mod make;

// Re-exported for tests
#[doc(hidden)]
pub use check_tailwind_syntax as syntax;

pub type TailwindSyntaxTreeBuilder = TreeBuilder<'static, TailwindLanguage, TailwindSyntaxFactory>;
