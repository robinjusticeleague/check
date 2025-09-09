use crate::FormatBogusNodeRule;
use check_js_syntax::JsBogusParameter;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsBogusParameter;

impl FormatBogusNodeRule<JsBogusParameter> for FormatJsBogusParameter {}
