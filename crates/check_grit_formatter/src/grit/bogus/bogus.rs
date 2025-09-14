use crate::FormatBogusNodeRule;
use check_grit_syntax::GritBogus;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritBogus;
impl FormatBogusNodeRule<GritBogus> for FormatGritBogus {}
