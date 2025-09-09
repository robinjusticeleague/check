use crate::prelude::*;

use check_formatter::write;
use check_js_syntax::JsObjectBindingPatternRest;
use check_js_syntax::JsObjectBindingPatternRestFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsObjectBindingPatternRest;

impl FormatNodeRule<JsObjectBindingPatternRest> for FormatJsObjectBindingPatternRest {
    fn fmt_fields(
        &self,
        node: &JsObjectBindingPatternRest,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsObjectBindingPatternRestFields {
            dotdotdot_token,
            binding,
        } = node.as_fields();

        write![f, [dotdotdot_token.format(), binding.format(),]]
    }
}
