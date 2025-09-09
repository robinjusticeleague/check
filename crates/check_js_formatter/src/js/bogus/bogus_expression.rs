use crate::FormatBogusNodeRule;
use check_js_syntax::JsBogusExpression;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsBogusExpression;

impl FormatBogusNodeRule<JsBogusExpression> for FormatJsBogusExpression {}
