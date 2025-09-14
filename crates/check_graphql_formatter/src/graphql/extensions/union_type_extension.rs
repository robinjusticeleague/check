use crate::prelude::*;
use check_formatter::{format_args, write};
use check_graphql_syntax::{GraphqlUnionTypeExtension, GraphqlUnionTypeExtensionFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlUnionTypeExtension;
impl FormatNodeRule<GraphqlUnionTypeExtension> for FormatGraphqlUnionTypeExtension {
    fn fmt_fields(
        &self,
        node: &GraphqlUnionTypeExtension,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        let GraphqlUnionTypeExtensionFields {
            extend_token,
            union_token,
            name,
            directives,
            union_members,
        } = node.as_fields();

        write![
            f,
            [group(&format_args!(
                extend_token.format(),
                space(),
                union_token.format(),
                space(),
                name.format(),
                directives.format(),
                union_members.format()
            ))]
        ]
    }
}
