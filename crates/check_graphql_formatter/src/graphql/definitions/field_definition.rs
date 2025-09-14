use crate::prelude::*;
use check_formatter::write;
use check_graphql_syntax::{GraphqlFieldDefinition, GraphqlFieldDefinitionFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlFieldDefinition;
impl FormatNodeRule<GraphqlFieldDefinition> for FormatGraphqlFieldDefinition {
    fn fmt_fields(
        &self,
        node: &GraphqlFieldDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        let GraphqlFieldDefinitionFields {
            description,
            name,
            arguments,
            colon_token,
            ty,
            directives,
        } = node.as_fields();

        if let Some(description) = description {
            write!(f, [description.format(), hard_line_break(),])?;
        }

        write!(
            f,
            [
                name.format(),
                arguments.format(),
                colon_token.format(),
                space(),
                ty.format(),
                directives.format(),
            ]
        )
    }
}
