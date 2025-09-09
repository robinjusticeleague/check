use crate::FormatBogusNodeRule;
use check_grit_syntax::GritBogusContainer;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritBogusContainer;
impl FormatBogusNodeRule<GritBogusContainer> for FormatGritBogusContainer {}
