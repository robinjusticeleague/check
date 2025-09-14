use crate::{MigrationAction, declare_migration};
use check_analyze::context::RuleContext;
use check_analyze::{Ast, Rule, RuleAction, RuleDiagnostic};
use check_console::markup;
use check_diagnostics::{Applicability, category};
use check_json_factory::make::{ident, json_string_value};
use check_json_syntax::{JsonMember, TextRange};
use check_rowan::{AstNode, BatchMutationExt};
use std::env;

pub(crate) const CHECK_VERSION: &str = match option_env!("CHECK_VERSION") {
    Some(version) => version,
    None => env!("CARGO_PKG_VERSION"),
};

declare_migration! {
    pub(crate) Schema {
        version: "1.5.0",
        name: "schema",
    }
}

impl Rule for Schema {
    type Query = Ast<JsonMember>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let node_text = node.name().ok()?.inner_string_text().ok()?;
        let member_value = node.value().ok()?;
        if node_text.text() == "$schema" {
            let string_value = member_value.as_json_string_value()?;
            let value = string_value.inner_string_text().ok()?;
            let value = value
                .text()
                .strip_prefix("https://checkjs.dev/schemas/")?
                .strip_suffix("/schema.json");

            if let Some(current_version) = value
                && current_version != CHECK_VERSION
            {
                return Some(string_value.range());
            }
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            category!("migrate"),
            state,
            markup! {
                "The version of the schema is outdated."
            }
            .to_owned(),
        ).note(markup!{
            "Having an old version of the schema won't allow you to see new options or deprecated ones."
        }))
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<MigrationAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        let schema = format!("\"https://checkjs.dev/schemas/{CHECK_VERSION}/schema.json\"");

        let new_node = json_string_value(ident(&schema));
        let member_value = node.value().ok()?;
        let member_value = member_value.as_json_string_value()?;
        mutation.replace_node(member_value.clone(), new_node);

        Some(RuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            Applicability::Always,
            markup! {
                "Update the URL."
            }
            .to_owned(),
            mutation,
        ))
    }
}
