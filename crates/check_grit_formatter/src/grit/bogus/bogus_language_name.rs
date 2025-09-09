use crate::FormatBogusNodeRule;
use check_grit_syntax::GritBogusLanguageName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritBogusLanguageName;
impl FormatBogusNodeRule<GritBogusLanguageName> for FormatGritBogusLanguageName {}
