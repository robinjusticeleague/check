//! Generated file, do not edit by hand, see `xtask/codegen`

//! Generated file, do not edit by hand, see `xtask/codegen`

use check_analyze::declare_lint_group;
pub mod no_check_first_exception;
pub mod no_duplicate_object_keys;
pub mod no_quickfix_check;
pub mod use_check_ignore_folder;
declare_lint_group! { pub Suspicious { name : "suspicious" , rules : [self :: no_check_first_exception :: NoCheckFirstException , self :: no_duplicate_object_keys :: NoDuplicateObjectKeys , self :: no_quickfix_check :: NoQuickfixCheck , self :: use_check_ignore_folder :: UseCheckIgnoreFolder ,] } }
