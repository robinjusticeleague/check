use crate::FormatBogusNodeRule;
use check_grit_syntax::GritBogusDefinition;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritBogusDefinition;
impl FormatBogusNodeRule<GritBogusDefinition> for FormatGritBogusDefinition {}
