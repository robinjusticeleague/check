use crate::prelude::*;
use check_formatter::separated::TrailingSeparator;
use check_grit_syntax::GritNamedArgList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritNamedArgList;
impl FormatRule<GritNamedArgList> for FormatGritNamedArgList {
    type Context = GritFormatContext;
    fn fmt(&self, node: &GritNamedArgList, f: &mut GritFormatter) -> FormatResult<()> {
        let mut join = f.join_with(space());

        join.entries(
            node.format_separated(",")
                .with_trailing_separator(TrailingSeparator::Omit),
        );

        join.finish()
    }
}
