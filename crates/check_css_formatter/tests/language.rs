use check_css_formatter::CssFormatLanguage;
use check_css_formatter::context::CssFormatContext;
use check_css_parser::{CssParserOptions, parse_css};
use check_css_syntax::{CssFileSource, CssLanguage};
use check_formatter_test::TestFormatLanguage;
use check_fs::CheckPath;
use check_parser::AnyParse;
use check_service::{
    settings::{ServiceLanguage, Settings},
    workspace::DocumentFileSource,
};

#[derive(Default)]
pub struct CssTestFormatLanguage {
    _source_type: CssFileSource,
}

impl TestFormatLanguage for CssTestFormatLanguage {
    type ServiceLanguage = CssLanguage;
    type Context = CssFormatContext;
    type FormatLanguage = CssFormatLanguage;

    fn parse(&self, text: &str) -> AnyParse {
        let options = CssParserOptions::default()
            .allow_wrong_line_comments()
            .allow_css_modules();

        parse_css(text, options).into()
    }

    fn to_format_language(
        &self,
        settings: &Settings,
        file_source: &DocumentFileSource,
    ) -> Self::FormatLanguage {
        let language_settings = &settings.languages.css.formatter;
        let options = Self::ServiceLanguage::resolve_format_options(
            &settings.formatter,
            &settings.override_settings,
            language_settings,
            &CheckPath::new(""),
            file_source,
        );
        CssFormatLanguage::new(options)
    }
}
