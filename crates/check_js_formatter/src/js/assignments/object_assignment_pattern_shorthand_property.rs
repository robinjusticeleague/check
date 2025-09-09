use crate::prelude::*;
use check_formatter::write;

use check_js_syntax::JsObjectAssignmentPatternShorthandProperty;
use check_js_syntax::JsObjectAssignmentPatternShorthandPropertyFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsObjectAssignmentPatternShorthandProperty;

impl FormatNodeRule<JsObjectAssignmentPatternShorthandProperty>
    for FormatJsObjectAssignmentPatternShorthandProperty
{
    fn fmt_fields(
        &self,
        node: &JsObjectAssignmentPatternShorthandProperty,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsObjectAssignmentPatternShorthandPropertyFields { identifier, init } =
            node.as_fields();

        write!(f, [identifier.format()?,])?;

        if let Some(init) = init {
            write!(f, [space(), init.format()])?;
        }
        Ok(())
    }
}
