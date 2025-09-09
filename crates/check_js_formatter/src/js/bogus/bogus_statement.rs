use crate::FormatBogusNodeRule;
use check_js_syntax::JsBogusStatement;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsBogusStatement;

impl FormatBogusNodeRule<JsBogusStatement> for FormatJsBogusStatement {}
