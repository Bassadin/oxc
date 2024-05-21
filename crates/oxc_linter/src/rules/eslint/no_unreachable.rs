use oxc_ast::{ast::VariableDeclarationKind, AstKind};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_semantic::{
    petgraph::{self, graph::NodeIndex},
    pg::neighbors_filtered_by_edge_weight,
    AstNode, EdgeType,
};
use oxc_span::{GetSpan, Span};

use crate::{context::LintContext, rule::Rule};

fn no_unreachable_diagnostic(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::error("eslint(no-unreachable): Unreachable code.").with_labels([span.into()])
}

/// <https://github.com/eslint/eslint/blob/069aa680c78b8516b9a1b568519f1d01e74fb2a2/lib/rules/no-unreachable.js#L196>
#[derive(Debug, Default, Clone)]
pub struct NoUnreachable;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallow unreachable code after `return`, `throw`, `continue`, and `break` statements
    ///
    NoUnreachable,
    correctness
);

impl Rule for NoUnreachable {
    fn run(&self, node: &AstNode, ctx: &LintContext) {
        // exit early if we are not visiting a statement.
        if !node.kind().is_statement() {
            return;
        }

        if matches! {
            node.kind(),
            AstKind::VariableDeclaration(decl)
                if matches!(decl.kind, VariableDeclarationKind::Var) && !decl.has_init()
        } {
            // Skip `var` declarations without any initialization,
            // These work because of the JavaScript hoisting rules.
            return;
        }

        let Some(parent) = ctx.nodes().parent_node(node.id()) else { unreachable!("?") };
        if is_unreachable(ctx, node.cfg_ix(), parent.cfg_ix()) {
            return ctx.diagnostic(no_unreachable_diagnostic(node.kind().span()));
        }
        // dbg!(node.kind().debug_name(), node.cfg_ix(), &ctx.semantic().cfg().graph);
    }
}

fn is_unreachable(ctx: &LintContext, target_ix: NodeIndex, from_ix: NodeIndex) -> bool {
    let msg = format!("is_unreachable {target_ix:?} from: {from_ix:?}");
    dbg!(msg);
    let cfg = ctx.semantic().cfg();
    if !petgraph::algo::has_path_connecting(&cfg.graph, from_ix, target_ix, None) {
        return true;
    }

    neighbors_filtered_by_edge_weight(
        &cfg.graph,
        from_ix,
        &|e| match e {
            EdgeType::Normal => None,
            EdgeType::Backedge | EdgeType::NewFunction => Some(None),
        },
        &mut |ix: &NodeIndex, _| {
            if target_ix == *ix {
                return (Some(true), false);
            }

            let connected = !petgraph::algo::has_path_connecting(&cfg.graph, *ix, target_ix, None);
            dbg!(ix, target_ix, connected);

            (None, connected)
        },
    )
    .iter()
    .any(|it| it.is_some_and(|val| !val))
}

#[test]
fn test() {
    use crate::tester::Tester;

    let pass = vec![
        "function foo() { function bar() { return 1; } return bar(); }",
        "function foo() { return bar(); function bar() { return 1; } }",
        "function foo() { return x; var x; }",
        "function foo() { var x = 1; var y = 2; }",
        "function foo() { var x = 1; var y = 2; return; }",
        "while (true) { switch (foo) { case 1: x = 1; x = 2;} }",
        "while (true) { break; var x; }",
        "while (true) { continue; var x, y; }",
        "while (true) { throw 'message'; var x; }",
        "while (true) { if (true) break; var x = 1; }",
        "while (true) continue;",
        "switch (foo) { case 1: break; var x; }",
        "switch (foo) { case 1: break; var x; default: throw true; };",
        "const arrow_direction = arrow => {  switch (arrow) { default: throw new Error();  };}",
        "var x = 1; y = 2; throw 'uh oh'; var y;",
        "function foo() { var x = 1; if (x) { return; } x = 2; }",
        "function foo() { var x = 1; if (x) { } else { return; } x = 2; }",
        "function foo() { var x = 1; switch (x) { case 0: break; default: return; } x = 2; }",
        "function foo() { var x = 1; while (x) { return; } x = 2; }",
        "function foo() { var x = 1; for (x in {}) { return; } x = 2; }",
        "function foo() { var x = 1; try { return; } finally { x = 2; } }",
        "function foo() { var x = 1; for (;;) { if (x) break; } x = 2; }",
        "A: { break A; } foo()",
        "function* foo() { try { yield 1; return; } catch (err) { return err; } }",
        "function foo() { try { bar(); return; } catch (err) { return err; } }",
        "function foo() { try { a.b.c = 1; return; } catch (err) { return err; } }",
        "class C { foo = reachable; }",
        "class C { foo = reachable; constructor() {} }",
        "class C extends B { foo = reachable; }",
        "class C extends B { foo = reachable; constructor() { super(); } }",
        "class C extends B { static foo = reachable; constructor() {} }",
    ];

    let fail = vec![
        //[{ messageId: "unreachableCode", type: "VariableDeclaration" }]
        "function foo() { return x; var x = 1; }",
        //[{ messageId: "unreachableCode", type: "VariableDeclaration" }]
        "function foo() { return x; var x, y = 1; }",
        "while (true) { break; var x = 1; }",
        //[{ messageId: "unreachableCode", type: "VariableDeclaration" }]
        "while (true) { continue; var x = 1; }",
        //[{ messageId: "unreachableCode", type: "ExpressionStatement" }]
        "function foo() { return; x = 1; }",
        //[{ messageId: "unreachableCode", type: "ExpressionStatement" }]
        "function foo() { throw error; x = 1; }",
        //[{ messageId: "unreachableCode", type: "ExpressionStatement" }]
        "while (true) { break; x = 1; }",
        //[{ messageId: "unreachableCode", type: "ExpressionStatement" }]
        "while (true) { continue; x = 1; }",
        //[{ messageId: "unreachableCode", type: "ExpressionStatement" }]
        "function foo() { switch (foo) { case 1: return; x = 1; } }",
        //[{ messageId: "unreachableCode", type: "ExpressionStatement" }]
        "function foo() { switch (foo) { case 1: throw e; x = 1; } }",
        //[{ messageId: "unreachableCode", type: "ExpressionStatement" }]
        "while (true) { switch (foo) { case 1: break; x = 1; } }",
        //[{ messageId: "unreachableCode", type: "ExpressionStatement" }]
        "while (true) { switch (foo) { case 1: continue; x = 1; } }",
        //[{ messageId: "unreachableCode", type: "VariableDeclaration" }]
        "var x = 1; throw 'uh oh'; var y = 2;",
        // [{ messageId: "unreachableCode", type: "ExpressionStatement" }]
        "function foo() { var x = 1; if (x) { return; } else { throw e; } x = 2; }",
        //[{ messageId: "unreachableCode", type: "ExpressionStatement" }]
        "function foo() { var x = 1; if (x) return; else throw -1; x = 2; }",
        //[{ messageId: "unreachableCode", type: "ExpressionStatement" }]
        "function foo() { var x = 1; try { return; } finally {} x = 2; }",
        //[{ messageId: "unreachableCode", type: "ExpressionStatement" }]
        "function foo() { var x = 1; try { } finally { return; } x = 2; }",
        //[{ messageId: "unreachableCode", type: "ExpressionStatement" }]
        "function foo() { var x = 1; do { return; } while (x); x = 2; }",
        // [{ messageId: "unreachableCode", type: "ExpressionStatement" }]
        "function foo() { var x = 1; while (x) { if (x) break; else continue; x = 2; } }",
        //[{ messageId: "unreachableCode", type: "ExpressionStatement" }]
        "function foo() { var x = 1; for (;;) { if (x) continue; } x = 2; }",
        //[{ messageId: "unreachableCode", type: "ExpressionStatement" }]
        "function foo() { var x = 1; while (true) { } x = 2; }",
    ];

    Tester::new(NoUnreachable::NAME, pass, fail).test_and_snapshot();
}
