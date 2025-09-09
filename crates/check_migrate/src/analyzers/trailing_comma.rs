use crate::{MigrationAction, declare_migration};
use check_analyze::context::RuleContext;
use check_analyze::{Ast, Rule, RuleAction, RuleDiagnostic};
use check_console::markup;
use check_diagnostics::{Applicability, category};
use check_json_factory::make::{json_member_name, json_string_literal};
use check_json_syntax::{JsonMember, JsonMemberName};
use check_rowan::{AstNode, BatchMutationExt};

declare_migration! {
    pub(crate) TrailingComma {
        version: "2.0.0",
        name: "trailingComma",
    }
}

impl Rule for TrailingComma {
    type Query = Ast<JsonMember>;
    type State = JsonMemberName;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let name = node.name().ok()?;
        let node_text = name.inner_string_text().ok()?;
        if node_text.text() == "trailingComma" {
            return Some(name);
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            category!("migrate"),
            state.range(),
            markup! {
                "The option trailingComma is removed. "
            }
            .to_owned(),
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<MigrationAction> {
        let mut mutation = ctx.root().begin();
        let new_name = json_member_name(json_string_literal("trailingCommas"));
        mutation.replace_node(state.clone(), new_name);

        Some(RuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            Applicability::Always,
            markup! {
                "Use the option trailingCommas instead."
            }
            .to_owned(),
            mutation,
        ))
    }
}
