/// Configuration related to the
/// [JSX A11y Eslint plugin](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y).
///
/// Also, the module includes implementation to convert rule options to Check's rule options.
use check_deserialize_macros::Deserializable;
use check_rule_options::use_valid_aria_role;

#[derive(Debug, Default, Deserializable)]
pub(crate) struct AriaRoleOptions {
    allowed_invalid_roles: Box<[Box<str>]>,
    #[deserializable(rename = "ignoreNonDOM")]
    ignore_non_dom: bool,
}
impl From<AriaRoleOptions> for use_valid_aria_role::UseValidAriaRoleOptions {
    fn from(val: AriaRoleOptions) -> Self {
        Self {
            allow_invalid_roles: val.allowed_invalid_roles,
            ignore_non_dom: val.ignore_non_dom,
        }
    }
}
