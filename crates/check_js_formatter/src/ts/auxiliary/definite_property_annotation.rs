use crate::prelude::*;

use check_formatter::write;
use check_js_syntax::TsDefinitePropertyAnnotation;
use check_js_syntax::TsDefinitePropertyAnnotationFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsDefinitePropertyAnnotation;

impl FormatNodeRule<TsDefinitePropertyAnnotation> for FormatTsDefinitePropertyAnnotation {
    fn fmt_fields(
        &self,
        node: &TsDefinitePropertyAnnotation,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsDefinitePropertyAnnotationFields {
            excl_token,
            type_annotation,
        } = node.as_fields();

        write![f, [excl_token.format(), type_annotation.format()]]
    }
}
