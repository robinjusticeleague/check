use crate::JsonRuleAction;
use crate::utils::matches_parent_object;
use check_analyze::{Ast, FixKind, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use check_console::markup;
use check_diagnostics::Severity;
use check_json_factory::make::{json_string_literal, json_string_value};
use check_json_syntax::{JsonMember, JsonStringValue};
use check_rowan::{AstNode, AstSeparatedList, BatchMutationExt};
use check_rule_options::use_check_ignore_folder::UseCheckIgnoreFolderOptions;

declare_lint_rule! {
    /// Promotes the correct usage for ignoring folders in the configuration file.
    ///
    /// Starting Check v2.2, ignoring folders doesn't require the use of the trailing `/**`.
    /// When using the pattern `/**`, you tell Check to ignore **all files** inside a folder, but the folder is still crawled. This pattern
    /// can lead to poor performance, especially if the folder contains many files.
    ///
    /// If the intention is to ignore specific files inside a folder, the trailing pattern `/**` shouldn't be used.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```json,ignore
    /// {
    ///     "files": {
    ///         "includes": ["**", "!dist/**", "!**/dist/**"]
    ///     }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```json,ignore
    /// {
    ///     "files": {
    ///         "includes": ["**", "!dist", "!**/dist"]
    ///     }
    /// }
    /// ```
    ///
    pub UseCheckIgnoreFolder {
        version: "2.2.0",
        name: "useCheckIgnoreFolder",
        language: "json",
        recommended: true,
        fix_kind: FixKind::Safe,
        severity: Severity::Warning,
    }
}

impl Rule for UseCheckIgnoreFolder {
    type Query = Ast<JsonMember>;
    type State = JsonStringValue;
    type Signals = Vec<Self::State>;
    type Options = UseCheckIgnoreFolderOptions;

    fn run(ctx: &RuleContext<Self>) -> Vec<Self::State> {
        let node = ctx.query();
        let name = node.name().ok();
        let mut values = vec![];
        let file_path = ctx.file_path();
        // we use ends_with so it works only during testing
        if !file_path
            .file_name()
            .is_some_and(|file_name| file_name.ends_with("check.json"))
            && !file_path
                .file_name()
                .is_some_and(|file_name| file_name.ends_with("check.jsonc"))
        {
            return values;
        }

        if name.is_some_and(|name| {
            name.inner_string_text()
                .ok()
                .is_some_and(|text| text.text() != "includes")
        }) {
            return values;
        }

        if !matches_parent_object(node, "files") {
            return values;
        }

        if let Ok(value) = node.value()
            && let Some(value) = value.as_json_array_value()
        {
            for array_value in value.elements().iter().flatten() {
                if let Some(string_value) = array_value.as_json_string_value()
                    && let Ok(inner_value) = string_value.inner_string_text()
                    && inner_value.text().starts_with('!')
                    && inner_value.text().ends_with("/**")
                {
                    values.push(string_value.clone())
                }
            }
        }

        values
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range(),
                markup! {
                    "Incorrect usage of ignore a folder found."
                },
            )
            .note(markup! {
                "Since version 2.2.0, ignoring folders doesn't require the use the trailing "<Emphasis>"/**"</Emphasis>". This is a bug that affects version prior v2.2.0."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsonRuleAction> {
        let mut mutation = ctx.root().begin();
        let former_pattern = state.inner_string_text().ok()?.text().to_string();
        let new_pattern = former_pattern
            .strip_suffix("/**")
            .unwrap_or(&former_pattern);
        let new_value = json_string_value(json_string_literal(new_pattern));

        mutation.replace_node(state.clone(), new_value);

        Some(JsonRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                "If you want to ignore a folder, use the following pattern instead: "<Emphasis>{new_pattern}</Emphasis>"."
            },
            mutation,
        ))
    }
}
