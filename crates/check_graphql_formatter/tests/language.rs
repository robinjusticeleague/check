use check_formatter_test::TestFormatLanguage;
use check_fs::CheckPath;
use check_graphql_formatter::GraphqlFormatLanguage;
use check_graphql_formatter::context::GraphqlFormatContext;
use check_graphql_parser::parse_graphql;
use check_graphql_syntax::{GraphqlFileSource, GraphqlLanguage};
use check_parser::AnyParse;
use check_service::{
    settings::{ServiceLanguage, Settings},
    workspace::DocumentFileSource,
};

#[derive(Default)]
pub struct GraphqlTestFormatLanguage {
    _source_type: GraphqlFileSource,
}

impl TestFormatLanguage for GraphqlTestFormatLanguage {
    type ServiceLanguage = GraphqlLanguage;
    type Context = GraphqlFormatContext;
    type FormatLanguage = GraphqlFormatLanguage;

    fn parse(&self, text: &str) -> AnyParse {
        let parse = parse_graphql(text);

        AnyParse::new(parse.syntax().as_send().unwrap(), parse.into_diagnostics())
    }

    fn to_format_language(
        &self,
        settings: &Settings,
        file_source: &DocumentFileSource,
    ) -> Self::FormatLanguage {
        let language_settings = &settings.languages.graphql.formatter;
        let options = Self::ServiceLanguage::resolve_format_options(
            &settings.formatter,
            &settings.override_settings,
            language_settings,
            &CheckPath::new(""),
            file_source,
        );
        GraphqlFormatLanguage::new(options)
    }
}
