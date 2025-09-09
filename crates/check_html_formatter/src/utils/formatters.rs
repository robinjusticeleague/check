use crate::prelude::*;
use crate::{HtmlFormatter, context::HtmlFormatContext};
use check_formatter::write;
use check_formatter::{Format, FormatResult};
use check_html_syntax::HtmlLanguage;
use check_rowan::{Language, SyntaxToken};
use check_string_case::StrLikeExtension;
use std::borrow::Cow;

// TODO: deduplicate with CSS formatter's version of this, move to `check_formatter`.
pub(crate) struct FormatTokenAsLowercase<L: Language> {
    token: SyntaxToken<L>,
}

impl<L: Language> From<SyntaxToken<L>> for FormatTokenAsLowercase<L> {
    fn from(value: SyntaxToken<L>) -> Self {
        Self { token: value }
    }
}

impl Format<HtmlFormatContext> for FormatTokenAsLowercase<HtmlLanguage> {
    fn fmt(&self, f: &mut HtmlFormatter) -> FormatResult<()> {
        let original = self.token.text_trimmed();
        match original.to_ascii_lowercase_cow() {
            Cow::Borrowed(_) => write!(f, [self.token.format()]),
            Cow::Owned(lowercase) => write!(
                f,
                [format_replaced(
                    &self.token,
                    &dynamic_text(&lowercase, self.token.text_trimmed_range().start()),
                )]
            ),
        }
    }
}
