//! This module provides important and useful traits to help to format tokens and nodes
//! when implementing a syntax formatter.

pub(crate) use crate::{
    AsFormat as _, FormatNodeRule, FormattedIterExt, JsFormatContext, JsFormatter,
    comments::JsComments, format_number_token, format_or_verbatim, format_removed, format_replaced,
    on_removed, on_skipped,
};
pub use check_formatter::prelude::*;
pub use check_formatter::separated::TrailingSeparator;
pub use check_rowan::{AstNode as _, AstNodeList as _, AstSeparatedList as _};

pub(crate) use crate::separated::FormatAstSeparatedListExtension;
