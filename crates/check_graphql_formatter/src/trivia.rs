use crate::FormatGraphqlSyntaxToken;
use crate::prelude::GraphqlFormatContext;
use check_formatter::formatter::Formatter;
use check_formatter::trivia::FormatToken;
use check_formatter::{Format, FormatResult};
use check_graphql_syntax::GraphqlSyntaxToken;

pub(crate) struct FormatRemoved<'a> {
    token: &'a GraphqlSyntaxToken,
}

pub(crate) fn format_removed(token: &GraphqlSyntaxToken) -> FormatRemoved<'_> {
    FormatRemoved { token }
}

impl<'a> Format<GraphqlFormatContext> for FormatRemoved<'a> {
    fn fmt(&self, f: &mut Formatter<GraphqlFormatContext>) -> FormatResult<()> {
        FormatGraphqlSyntaxToken.format_removed(self.token, f)
    }
}
