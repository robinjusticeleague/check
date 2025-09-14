use check_formatter_test::TestFormatLanguage;
use check_fs::CheckPath;
use check_html_formatter::HtmlFormatLanguage;
use check_html_formatter::context::HtmlFormatContext;
use check_html_parser::{HtmlParseOptions, parse_html};
use check_html_syntax::{HtmlFileSource, HtmlLanguage};
use check_parser::AnyParse;
use check_service::{
    settings::{ServiceLanguage, Settings},
    workspace::DocumentFileSource,
};

pub struct HtmlTestFormatLanguage {
    source_type: HtmlFileSource,
}

impl HtmlTestFormatLanguage {
    pub fn new(source_type: HtmlFileSource) -> Self {
        Self { source_type }
    }
}

impl TestFormatLanguage for HtmlTestFormatLanguage {
    type ServiceLanguage = HtmlLanguage;
    type Context = HtmlFormatContext;
    type FormatLanguage = HtmlFormatLanguage;

    fn parse(&self, text: &str) -> AnyParse {
        parse_html(text, HtmlParseOptions::from(&self.source_type)).into()
    }

    fn to_format_language(
        &self,
        settings: &Settings,
        file_source: &DocumentFileSource,
    ) -> Self::FormatLanguage {
        let options = Self::ServiceLanguage::resolve_format_options(
            &settings.formatter,
            &settings.override_settings,
            &settings.languages.html.formatter,
            &CheckPath::new(""),
            file_source,
        );
        HtmlFormatLanguage::new(options)
    }
}
