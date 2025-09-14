use crate::prelude::*;

use check_formatter::write;
use check_js_syntax::JsArrayBindingPatternElement;
use check_js_syntax::JsArrayBindingPatternElementFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsArrayBindingPatternElement;

impl FormatNodeRule<JsArrayBindingPatternElement> for FormatJsArrayBindingPatternElement {
    fn fmt_fields(
        &self,
        node: &JsArrayBindingPatternElement,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsArrayBindingPatternElementFields { pattern, init } = node.as_fields();

        write!(f, [pattern.format()])?;
        if let Some(init) = init {
            write!(f, [space(), init.format()])?;
        }

        Ok(())
    }
}
