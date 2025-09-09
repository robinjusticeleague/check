use crate::prelude::*;
use check_formatter::write;
use check_graphql_syntax::{GraphqlEnumValue, GraphqlEnumValueFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlEnumValue;
impl FormatNodeRule<GraphqlEnumValue> for FormatGraphqlEnumValue {
    fn fmt_fields(&self, node: &GraphqlEnumValue, f: &mut GraphqlFormatter) -> FormatResult<()> {
        let GraphqlEnumValueFields { value } = node.as_fields();

        write!(f, [value.format()])
    }
}
