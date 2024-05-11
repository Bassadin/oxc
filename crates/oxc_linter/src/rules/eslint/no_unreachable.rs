use oxc_ast::{ast::VariableDeclarationKind, AstKind};
use oxc_diagnostics::{
    miette::{self, Diagnostic},
    thiserror::Error,
};
use oxc_macros::declare_oxc_lint;
use oxc_semantic::{
    petgraph::{self, graph::NodeIndex},
    AstNode,
};
use oxc_span::{GetSpan, Span};

use crate::{context::LintContext, rule::Rule};

#[derive(Debug, Error, Diagnostic)]
#[error("eslint(no-unreachable): Unreachable code.")]
#[diagnostic(severity(warning))]
struct NoUnreachableDiagnostic(#[label] pub Span);

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
            return ctx.diagnostic(NoUnreachableDiagnostic(node.kind().span()));
        }
        // dbg!(node.kind().debug_name(), node.cfg_ix(), &ctx.semantic().cfg().graph);
    }
}

fn is_unreachable(ctx: &LintContext, node_cfg_ix: NodeIndex, parent_cfg_ix: NodeIndex) -> bool {
    !petgraph::algo::has_path_connecting(
        &ctx.semantic().cfg().graph,
        parent_cfg_ix,
        node_cfg_ix,
        None,
    )
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
