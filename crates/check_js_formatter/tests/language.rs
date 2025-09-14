use check_formatter_test::TestFormatLanguage;
use check_fs::CheckPath;
use check_js_formatter::JsFormatLanguage;
use check_js_formatter::context::JsFormatContext;
use check_js_parser::{JsParserOptions, parse};
use check_js_syntax::{JsFileSource, JsLanguage};
use check_parser::AnyParse;
use check_service::{
    settings::{ServiceLanguage, Settings},
    workspace::DocumentFileSource,
};

pub struct JsTestFormatLanguage {
    source_type: JsFileSource,
}

impl JsTestFormatLanguage {
    pub fn new(source_type: JsFileSource) -> Self {
        Self { source_type }
    }
}

impl TestFormatLanguage for JsTestFormatLanguage {
    type ServiceLanguage = JsLanguage;
    type Context = JsFormatContext;
    type FormatLanguage = JsFormatLanguage;

    fn parse(&self, text: &str) -> AnyParse {
        let options = JsParserOptions::default().with_parse_class_parameter_decorators();

        parse(text, self.source_type, options).into()
    }

    fn to_format_language(
        &self,
        settings: &Settings,
        file_source: &DocumentFileSource,
    ) -> Self::FormatLanguage {
        let language_settings = &settings.languages.javascript.formatter;
        let options = Self::ServiceLanguage::resolve_format_options(
            &settings.formatter,
            &settings.override_settings,
            language_settings,
            &CheckPath::new(""),
            file_source,
        );
        JsFormatLanguage::new(options)
    }
}
