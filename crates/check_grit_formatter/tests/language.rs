use check_formatter_test::TestFormatLanguage;
use check_fs::CheckPath;
use check_grit_formatter::{GritFormatLanguage, context::GritFormatContext};
use check_grit_parser::parse_grit;
use check_grit_syntax::GritLanguage;
use check_service::settings::ServiceLanguage;

#[derive(Default)]
pub struct GritTestFormatLanguage;

impl TestFormatLanguage for GritTestFormatLanguage {
    type ServiceLanguage = GritLanguage;
    type Context = GritFormatContext;
    type FormatLanguage = GritFormatLanguage;

    fn parse(&self, text: &str) -> check_parser::AnyParse {
        parse_grit(text).into()
    }

    fn to_format_language(
        &self,
        settings: &check_service::settings::Settings,
        file_source: &check_service::workspace::DocumentFileSource,
    ) -> Self::FormatLanguage {
        let language_settings = &settings.languages.grit.formatter;
        let options = Self::ServiceLanguage::resolve_format_options(
            &settings.formatter,
            &settings.override_settings,
            language_settings,
            &CheckPath::new(""),
            file_source,
        );
        GritFormatLanguage::new(options)
    }
}
