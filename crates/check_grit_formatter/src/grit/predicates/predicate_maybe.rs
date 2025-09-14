use crate::prelude::*;
use check_formatter::write;
use check_grit_syntax::{GritPredicateMaybe, GritPredicateMaybeFields};
use std::fmt::Debug;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateMaybe;
impl FormatNodeRule<GritPredicateMaybe> for FormatGritPredicateMaybe {
    fn fmt_fields(&self, node: &GritPredicateMaybe, f: &mut GritFormatter) -> FormatResult<()> {
        let GritPredicateMaybeFields {
            predicate,
            maybe_token,
        } = node.as_fields();
        write!(f, [maybe_token.format(), space(), predicate.format()])
    }
}
