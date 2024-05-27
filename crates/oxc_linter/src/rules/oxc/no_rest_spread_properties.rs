use oxc_ast::{ast::AssignmentTarget, AstKind};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::Span;

use crate::{context::LintContext, rule::Rule, AstNode};

fn no_rest_spread_properties_diagnostic(span0: Span, x1: &str) -> OxcDiagnostic {
    OxcDiagnostic::warn(format!("oxc(no-rest-spread-properties): {x1} are not allowed."))
        .with_labels([span0.into()])
}

#[derive(Debug, Default, Clone)]
pub struct NoRestSpreadProperties;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallow [Object Rest/Spread Properties](https://github.com/tc39/proposal-object-rest-spread#readme).
    ///
    /// ### Example
    ///
    /// ```javascript
    /// let { x, ...y } = z;
    /// let z = { x, ...y };
    /// ```
    NoRestSpreadProperties,
    restriction,
);

impl Rule for NoRestSpreadProperties {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        match node.kind() {
            AstKind::SpreadElement(spread_element) => {
                if ctx
                    .nodes()
                    .parent_kind(node.id())
                    .is_some_and(|parent| matches!(parent, AstKind::ObjectExpression(_)))
                {
                    ctx.diagnostic(no_rest_spread_properties_diagnostic(
                        spread_element.span,
                        "object spread property",
                    ));
                }
            }
            AstKind::BindingRestElement(rest_element) => {
                if ctx
                    .nodes()
                    .parent_kind(node.id())
                    .is_some_and(|parent| matches!(parent, AstKind::ObjectPattern(_)))
                {
                    ctx.diagnostic(no_rest_spread_properties_diagnostic(
                        rest_element.span,
                        "object rest property",
                    ));
                }
            }
            AstKind::AssignmentTarget(assign_target) => {
                let AssignmentTarget::ObjectAssignmentTarget(object_assign) = assign_target else {
                    return;
                };
                let Some(rest) = &object_assign.rest else {
                    return;
                };

                ctx.diagnostic(no_rest_spread_properties_diagnostic(
                    rest.span,
                    "object rest property",
                ));
            }
            _ => {}
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;

    // test case are copied from eslint-plugin-es:
    // https://github.com/mysticatea/eslint-plugin-es/blob/v1.4.1/tests/lib/rules/no-rest-spread-properties.js
    let pass = vec![
        "[...a]",
        "[...a] = foo",
        "({a: [...b]})",
        "({a: [...b]} = obj)",
        "function f(...a) {}",
        "f(...a)",
    ];

    let fail =
        vec!["({...a})", "({...a} = obj)", "for ({...a} in foo) {}", "function f({...a}) {}"];

    Tester::new(NoRestSpreadProperties::NAME, pass, fail).test_and_snapshot();
}
