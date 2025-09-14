use crate::prelude::*;
use check_formatter::write;

use check_js_syntax::JsArrayAssignmentPatternRestElement;
use check_js_syntax::JsArrayAssignmentPatternRestElementFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsArrayAssignmentPatternRestElement;

impl FormatNodeRule<JsArrayAssignmentPatternRestElement>
    for FormatJsArrayAssignmentPatternRestElement
{
    fn fmt_fields(
        &self,
        node: &JsArrayAssignmentPatternRestElement,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsArrayAssignmentPatternRestElementFields {
            dotdotdot_token,
            pattern,
        } = node.as_fields();

        write!(f, [dotdotdot_token.format(), pattern.format()])
    }
}
