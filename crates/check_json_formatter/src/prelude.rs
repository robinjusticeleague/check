//! This module provides important and useful traits to help to format tokens and nodes
//! when implementing the [crate::FormatNodeRule] trait.

#![allow(unused_imports)]
pub(crate) use crate::{
    AsFormat, FormatNodeRule, FormattedIterExt as _, IntoFormat, JsonFormatContext, JsonFormatter,
    format_number_token, format_removed, format_replaced, on_removed, on_skipped, verbatim::*,
};
pub(crate) use check_formatter::prelude::*;
pub(crate) use check_rowan::{AstNode as _, AstNodeList as _, AstSeparatedList as _};
