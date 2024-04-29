use super::{ast::*, cell::Token, SharedBox, TraverseCtx};

#[allow(unused_variables)]
pub trait Traverse<'a> {
    fn enter_program(
        &mut self,
        node: SharedBox<'a, TraversableProgram<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_program(
        &mut self,
        node: SharedBox<'a, TraversableProgram<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_expression(
        &mut self,
        node: TraversableExpression<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_expression(
        &mut self,
        node: TraversableExpression<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn visit_identifier_reference(
        &mut self,
        node: SharedBox<'a, TraversableIdentifierReference<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn visit_binding_identifier(
        &mut self,
        node: SharedBox<'a, TraversableBindingIdentifier<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn visit_label_identifier(
        &mut self,
        node: SharedBox<'a, TraversableLabelIdentifier<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn visit_this_expression(
        &mut self,
        node: SharedBox<'a, TraversableThisExpression>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_array_expression(
        &mut self,
        node: SharedBox<'a, TraversableArrayExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_array_expression(
        &mut self,
        node: SharedBox<'a, TraversableArrayExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_array_expression_element(
        &mut self,
        node: TraversableArrayExpressionElement<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_array_expression_element(
        &mut self,
        node: TraversableArrayExpressionElement<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn visit_elision(
        &mut self,
        node: SharedBox<'a, TraversableElision>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_object_expression(
        &mut self,
        node: SharedBox<'a, TraversableObjectExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_object_expression(
        &mut self,
        node: SharedBox<'a, TraversableObjectExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_object_property_kind(
        &mut self,
        node: TraversableObjectPropertyKind<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_object_property_kind(
        &mut self,
        node: TraversableObjectPropertyKind<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_object_property(
        &mut self,
        node: SharedBox<'a, TraversableObjectProperty<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_object_property(
        &mut self,
        node: SharedBox<'a, TraversableObjectProperty<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_property_key(
        &mut self,
        node: TraversablePropertyKey<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_property_key(
        &mut self,
        node: TraversablePropertyKey<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_template_literal(
        &mut self,
        node: SharedBox<'a, TraversableTemplateLiteral<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_template_literal(
        &mut self,
        node: SharedBox<'a, TraversableTemplateLiteral<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_tagged_template_expression(
        &mut self,
        node: SharedBox<'a, TraversableTaggedTemplateExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_tagged_template_expression(
        &mut self,
        node: SharedBox<'a, TraversableTaggedTemplateExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn visit_template_element(
        &mut self,
        node: SharedBox<'a, TraversableTemplateElement<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_member_expression(
        &mut self,
        node: TraversableMemberExpression<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_member_expression(
        &mut self,
        node: TraversableMemberExpression<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_computed_member_expression(
        &mut self,
        node: SharedBox<'a, TraversableComputedMemberExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_computed_member_expression(
        &mut self,
        node: SharedBox<'a, TraversableComputedMemberExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_static_member_expression(
        &mut self,
        node: SharedBox<'a, TraversableStaticMemberExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_static_member_expression(
        &mut self,
        node: SharedBox<'a, TraversableStaticMemberExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_private_field_expression(
        &mut self,
        node: SharedBox<'a, TraversablePrivateFieldExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_private_field_expression(
        &mut self,
        node: SharedBox<'a, TraversablePrivateFieldExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_call_expression(
        &mut self,
        node: SharedBox<'a, TraversableCallExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_call_expression(
        &mut self,
        node: SharedBox<'a, TraversableCallExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_new_expression(
        &mut self,
        node: SharedBox<'a, TraversableNewExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_new_expression(
        &mut self,
        node: SharedBox<'a, TraversableNewExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn visit_meta_property(
        &mut self,
        node: SharedBox<'a, TraversableMetaProperty<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_spread_element(
        &mut self,
        node: SharedBox<'a, TraversableSpreadElement<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_spread_element(
        &mut self,
        node: SharedBox<'a, TraversableSpreadElement<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_argument(
        &mut self,
        node: TraversableArgument<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_argument(
        &mut self,
        node: TraversableArgument<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_update_expression(
        &mut self,
        node: SharedBox<'a, TraversableUpdateExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_update_expression(
        &mut self,
        node: SharedBox<'a, TraversableUpdateExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_unary_expression(
        &mut self,
        node: SharedBox<'a, TraversableUnaryExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_unary_expression(
        &mut self,
        node: SharedBox<'a, TraversableUnaryExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_binary_expression(
        &mut self,
        node: SharedBox<'a, TraversableBinaryExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_binary_expression(
        &mut self,
        node: SharedBox<'a, TraversableBinaryExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_private_in_expression(
        &mut self,
        node: SharedBox<'a, TraversablePrivateInExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_private_in_expression(
        &mut self,
        node: SharedBox<'a, TraversablePrivateInExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_logical_expression(
        &mut self,
        node: SharedBox<'a, TraversableLogicalExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_logical_expression(
        &mut self,
        node: SharedBox<'a, TraversableLogicalExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_conditional_expression(
        &mut self,
        node: SharedBox<'a, TraversableConditionalExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_conditional_expression(
        &mut self,
        node: SharedBox<'a, TraversableConditionalExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_assignment_expression(
        &mut self,
        node: SharedBox<'a, TraversableAssignmentExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_assignment_expression(
        &mut self,
        node: SharedBox<'a, TraversableAssignmentExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_assignment_target(
        &mut self,
        node: TraversableAssignmentTarget<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_assignment_target(
        &mut self,
        node: TraversableAssignmentTarget<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_simple_assignment_target(
        &mut self,
        node: TraversableSimpleAssignmentTarget<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_simple_assignment_target(
        &mut self,
        node: TraversableSimpleAssignmentTarget<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_assignment_target_pattern(
        &mut self,
        node: TraversableAssignmentTargetPattern<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_assignment_target_pattern(
        &mut self,
        node: TraversableAssignmentTargetPattern<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_array_assignment_target(
        &mut self,
        node: SharedBox<'a, TraversableArrayAssignmentTarget<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_array_assignment_target(
        &mut self,
        node: SharedBox<'a, TraversableArrayAssignmentTarget<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_object_assignment_target(
        &mut self,
        node: SharedBox<'a, TraversableObjectAssignmentTarget<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_object_assignment_target(
        &mut self,
        node: SharedBox<'a, TraversableObjectAssignmentTarget<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_assignment_target_rest(
        &mut self,
        node: SharedBox<'a, TraversableAssignmentTargetRest<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_assignment_target_rest(
        &mut self,
        node: SharedBox<'a, TraversableAssignmentTargetRest<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_assignment_target_maybe_default(
        &mut self,
        node: TraversableAssignmentTargetMaybeDefault<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_assignment_target_maybe_default(
        &mut self,
        node: TraversableAssignmentTargetMaybeDefault<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_assignment_target_with_default(
        &mut self,
        node: SharedBox<'a, TraversableAssignmentTargetWithDefault<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_assignment_target_with_default(
        &mut self,
        node: SharedBox<'a, TraversableAssignmentTargetWithDefault<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_assignment_target_property(
        &mut self,
        node: TraversableAssignmentTargetProperty<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_assignment_target_property(
        &mut self,
        node: TraversableAssignmentTargetProperty<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_assignment_target_property_identifier(
        &mut self,
        node: SharedBox<'a, TraversableAssignmentTargetPropertyIdentifier<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_assignment_target_property_identifier(
        &mut self,
        node: SharedBox<'a, TraversableAssignmentTargetPropertyIdentifier<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_assignment_target_property_property(
        &mut self,
        node: SharedBox<'a, TraversableAssignmentTargetPropertyProperty<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_assignment_target_property_property(
        &mut self,
        node: SharedBox<'a, TraversableAssignmentTargetPropertyProperty<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_sequence_expression(
        &mut self,
        node: SharedBox<'a, TraversableSequenceExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_sequence_expression(
        &mut self,
        node: SharedBox<'a, TraversableSequenceExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn visit_super(
        &mut self,
        node: SharedBox<'a, TraversableSuper>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_await_expression(
        &mut self,
        node: SharedBox<'a, TraversableAwaitExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_await_expression(
        &mut self,
        node: SharedBox<'a, TraversableAwaitExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_chain_expression(
        &mut self,
        node: SharedBox<'a, TraversableChainExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_chain_expression(
        &mut self,
        node: SharedBox<'a, TraversableChainExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_chain_element(
        &mut self,
        node: TraversableChainElement<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_chain_element(
        &mut self,
        node: TraversableChainElement<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_parenthesized_expression(
        &mut self,
        node: SharedBox<'a, TraversableParenthesizedExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_parenthesized_expression(
        &mut self,
        node: SharedBox<'a, TraversableParenthesizedExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_statement(
        &mut self,
        node: TraversableStatement<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_statement(
        &mut self,
        node: TraversableStatement<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_directive(
        &mut self,
        node: SharedBox<'a, TraversableDirective<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_directive(
        &mut self,
        node: SharedBox<'a, TraversableDirective<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn visit_hashbang(
        &mut self,
        node: SharedBox<'a, TraversableHashbang<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_block_statement(
        &mut self,
        node: SharedBox<'a, TraversableBlockStatement<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_block_statement(
        &mut self,
        node: SharedBox<'a, TraversableBlockStatement<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_declaration(
        &mut self,
        node: TraversableDeclaration<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_declaration(
        &mut self,
        node: TraversableDeclaration<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_variable_declaration(
        &mut self,
        node: SharedBox<'a, TraversableVariableDeclaration<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_variable_declaration(
        &mut self,
        node: SharedBox<'a, TraversableVariableDeclaration<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_variable_declarator(
        &mut self,
        node: SharedBox<'a, TraversableVariableDeclarator<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_variable_declarator(
        &mut self,
        node: SharedBox<'a, TraversableVariableDeclarator<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_using_declaration(
        &mut self,
        node: SharedBox<'a, TraversableUsingDeclaration<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_using_declaration(
        &mut self,
        node: SharedBox<'a, TraversableUsingDeclaration<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn visit_empty_statement(
        &mut self,
        node: SharedBox<'a, TraversableEmptyStatement>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_expression_statement(
        &mut self,
        node: SharedBox<'a, TraversableExpressionStatement<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_expression_statement(
        &mut self,
        node: SharedBox<'a, TraversableExpressionStatement<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_if_statement(
        &mut self,
        node: SharedBox<'a, TraversableIfStatement<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_if_statement(
        &mut self,
        node: SharedBox<'a, TraversableIfStatement<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_do_while_statement(
        &mut self,
        node: SharedBox<'a, TraversableDoWhileStatement<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_do_while_statement(
        &mut self,
        node: SharedBox<'a, TraversableDoWhileStatement<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_while_statement(
        &mut self,
        node: SharedBox<'a, TraversableWhileStatement<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_while_statement(
        &mut self,
        node: SharedBox<'a, TraversableWhileStatement<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_for_statement(
        &mut self,
        node: SharedBox<'a, TraversableForStatement<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_for_statement(
        &mut self,
        node: SharedBox<'a, TraversableForStatement<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_for_statement_init(
        &mut self,
        node: TraversableForStatementInit<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_for_statement_init(
        &mut self,
        node: TraversableForStatementInit<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_for_in_statement(
        &mut self,
        node: SharedBox<'a, TraversableForInStatement<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_for_in_statement(
        &mut self,
        node: SharedBox<'a, TraversableForInStatement<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_for_of_statement(
        &mut self,
        node: SharedBox<'a, TraversableForOfStatement<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_for_of_statement(
        &mut self,
        node: SharedBox<'a, TraversableForOfStatement<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_for_statement_left(
        &mut self,
        node: TraversableForStatementLeft<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_for_statement_left(
        &mut self,
        node: TraversableForStatementLeft<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_continue_statement(
        &mut self,
        node: SharedBox<'a, TraversableContinueStatement<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_continue_statement(
        &mut self,
        node: SharedBox<'a, TraversableContinueStatement<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_break_statement(
        &mut self,
        node: SharedBox<'a, TraversableBreakStatement<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_break_statement(
        &mut self,
        node: SharedBox<'a, TraversableBreakStatement<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_return_statement(
        &mut self,
        node: SharedBox<'a, TraversableReturnStatement<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_return_statement(
        &mut self,
        node: SharedBox<'a, TraversableReturnStatement<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_with_statement(
        &mut self,
        node: SharedBox<'a, TraversableWithStatement<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_with_statement(
        &mut self,
        node: SharedBox<'a, TraversableWithStatement<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_switch_statement(
        &mut self,
        node: SharedBox<'a, TraversableSwitchStatement<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_switch_statement(
        &mut self,
        node: SharedBox<'a, TraversableSwitchStatement<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_switch_case(
        &mut self,
        node: SharedBox<'a, TraversableSwitchCase<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_switch_case(
        &mut self,
        node: SharedBox<'a, TraversableSwitchCase<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_labeled_statement(
        &mut self,
        node: SharedBox<'a, TraversableLabeledStatement<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_labeled_statement(
        &mut self,
        node: SharedBox<'a, TraversableLabeledStatement<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_throw_statement(
        &mut self,
        node: SharedBox<'a, TraversableThrowStatement<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_throw_statement(
        &mut self,
        node: SharedBox<'a, TraversableThrowStatement<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_try_statement(
        &mut self,
        node: SharedBox<'a, TraversableTryStatement<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_try_statement(
        &mut self,
        node: SharedBox<'a, TraversableTryStatement<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_catch_clause(
        &mut self,
        node: SharedBox<'a, TraversableCatchClause<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_catch_clause(
        &mut self,
        node: SharedBox<'a, TraversableCatchClause<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_catch_parameter(
        &mut self,
        node: SharedBox<'a, TraversableCatchParameter<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_catch_parameter(
        &mut self,
        node: SharedBox<'a, TraversableCatchParameter<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn visit_debugger_statement(
        &mut self,
        node: SharedBox<'a, TraversableDebuggerStatement>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_binding_pattern(
        &mut self,
        node: SharedBox<'a, TraversableBindingPattern<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_binding_pattern(
        &mut self,
        node: SharedBox<'a, TraversableBindingPattern<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_binding_pattern_kind(
        &mut self,
        node: TraversableBindingPatternKind<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_binding_pattern_kind(
        &mut self,
        node: TraversableBindingPatternKind<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_assignment_pattern(
        &mut self,
        node: SharedBox<'a, TraversableAssignmentPattern<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_assignment_pattern(
        &mut self,
        node: SharedBox<'a, TraversableAssignmentPattern<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_object_pattern(
        &mut self,
        node: SharedBox<'a, TraversableObjectPattern<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_object_pattern(
        &mut self,
        node: SharedBox<'a, TraversableObjectPattern<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_binding_property(
        &mut self,
        node: SharedBox<'a, TraversableBindingProperty<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_binding_property(
        &mut self,
        node: SharedBox<'a, TraversableBindingProperty<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_array_pattern(
        &mut self,
        node: SharedBox<'a, TraversableArrayPattern<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_array_pattern(
        &mut self,
        node: SharedBox<'a, TraversableArrayPattern<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_binding_rest_element(
        &mut self,
        node: SharedBox<'a, TraversableBindingRestElement<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_binding_rest_element(
        &mut self,
        node: SharedBox<'a, TraversableBindingRestElement<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_function(
        &mut self,
        node: SharedBox<'a, TraversableFunction<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_function(
        &mut self,
        node: SharedBox<'a, TraversableFunction<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_formal_parameters(
        &mut self,
        node: SharedBox<'a, TraversableFormalParameters<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_formal_parameters(
        &mut self,
        node: SharedBox<'a, TraversableFormalParameters<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_formal_parameter(
        &mut self,
        node: SharedBox<'a, TraversableFormalParameter<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_formal_parameter(
        &mut self,
        node: SharedBox<'a, TraversableFormalParameter<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_function_body(
        &mut self,
        node: SharedBox<'a, TraversableFunctionBody<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_function_body(
        &mut self,
        node: SharedBox<'a, TraversableFunctionBody<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_arrow_function_expression(
        &mut self,
        node: SharedBox<'a, TraversableArrowFunctionExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_arrow_function_expression(
        &mut self,
        node: SharedBox<'a, TraversableArrowFunctionExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_yield_expression(
        &mut self,
        node: SharedBox<'a, TraversableYieldExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_yield_expression(
        &mut self,
        node: SharedBox<'a, TraversableYieldExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_class(
        &mut self,
        node: SharedBox<'a, TraversableClass<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_class(
        &mut self,
        node: SharedBox<'a, TraversableClass<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_class_body(
        &mut self,
        node: SharedBox<'a, TraversableClassBody<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_class_body(
        &mut self,
        node: SharedBox<'a, TraversableClassBody<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_class_element(
        &mut self,
        node: TraversableClassElement<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_class_element(
        &mut self,
        node: TraversableClassElement<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_method_definition(
        &mut self,
        node: SharedBox<'a, TraversableMethodDefinition<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_method_definition(
        &mut self,
        node: SharedBox<'a, TraversableMethodDefinition<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_property_definition(
        &mut self,
        node: SharedBox<'a, TraversablePropertyDefinition<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_property_definition(
        &mut self,
        node: SharedBox<'a, TraversablePropertyDefinition<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn visit_private_identifier(
        &mut self,
        node: SharedBox<'a, TraversablePrivateIdentifier<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_static_block(
        &mut self,
        node: SharedBox<'a, TraversableStaticBlock<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_static_block(
        &mut self,
        node: SharedBox<'a, TraversableStaticBlock<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_module_declaration(
        &mut self,
        node: TraversableModuleDeclaration<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_module_declaration(
        &mut self,
        node: TraversableModuleDeclaration<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_accessor_property(
        &mut self,
        node: SharedBox<'a, TraversableAccessorProperty<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_accessor_property(
        &mut self,
        node: SharedBox<'a, TraversableAccessorProperty<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_import_expression(
        &mut self,
        node: SharedBox<'a, TraversableImportExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_import_expression(
        &mut self,
        node: SharedBox<'a, TraversableImportExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_import_declaration(
        &mut self,
        node: SharedBox<'a, TraversableImportDeclaration<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_import_declaration(
        &mut self,
        node: SharedBox<'a, TraversableImportDeclaration<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_import_declaration_specifier(
        &mut self,
        node: TraversableImportDeclarationSpecifier<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_import_declaration_specifier(
        &mut self,
        node: TraversableImportDeclarationSpecifier<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_import_specifier(
        &mut self,
        node: SharedBox<'a, TraversableImportSpecifier<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_import_specifier(
        &mut self,
        node: SharedBox<'a, TraversableImportSpecifier<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_import_default_specifier(
        &mut self,
        node: SharedBox<'a, TraversableImportDefaultSpecifier<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_import_default_specifier(
        &mut self,
        node: SharedBox<'a, TraversableImportDefaultSpecifier<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_import_namespace_specifier(
        &mut self,
        node: SharedBox<'a, TraversableImportNamespaceSpecifier<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_import_namespace_specifier(
        &mut self,
        node: SharedBox<'a, TraversableImportNamespaceSpecifier<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_with_clause(
        &mut self,
        node: SharedBox<'a, TraversableWithClause<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_with_clause(
        &mut self,
        node: SharedBox<'a, TraversableWithClause<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_import_attribute(
        &mut self,
        node: SharedBox<'a, TraversableImportAttribute<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_import_attribute(
        &mut self,
        node: SharedBox<'a, TraversableImportAttribute<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_import_attribute_key(
        &mut self,
        node: TraversableImportAttributeKey<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_import_attribute_key(
        &mut self,
        node: TraversableImportAttributeKey<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_export_named_declaration(
        &mut self,
        node: SharedBox<'a, TraversableExportNamedDeclaration<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_export_named_declaration(
        &mut self,
        node: SharedBox<'a, TraversableExportNamedDeclaration<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_export_default_declaration(
        &mut self,
        node: SharedBox<'a, TraversableExportDefaultDeclaration<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_export_default_declaration(
        &mut self,
        node: SharedBox<'a, TraversableExportDefaultDeclaration<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_export_all_declaration(
        &mut self,
        node: SharedBox<'a, TraversableExportAllDeclaration<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_export_all_declaration(
        &mut self,
        node: SharedBox<'a, TraversableExportAllDeclaration<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_export_specifier(
        &mut self,
        node: SharedBox<'a, TraversableExportSpecifier<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_export_specifier(
        &mut self,
        node: SharedBox<'a, TraversableExportSpecifier<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_export_default_declaration_kind(
        &mut self,
        node: TraversableExportDefaultDeclarationKind<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_export_default_declaration_kind(
        &mut self,
        node: TraversableExportDefaultDeclarationKind<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_module_export_name(
        &mut self,
        node: TraversableModuleExportName<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_module_export_name(
        &mut self,
        node: TraversableModuleExportName<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_jsx_element(
        &mut self,
        node: SharedBox<'a, TraversableJSXElement<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_jsx_element(
        &mut self,
        node: SharedBox<'a, TraversableJSXElement<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_jsx_opening_element(
        &mut self,
        node: SharedBox<'a, TraversableJSXOpeningElement<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_jsx_opening_element(
        &mut self,
        node: SharedBox<'a, TraversableJSXOpeningElement<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_jsx_closing_element(
        &mut self,
        node: SharedBox<'a, TraversableJSXClosingElement<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_jsx_closing_element(
        &mut self,
        node: SharedBox<'a, TraversableJSXClosingElement<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_jsx_fragment(
        &mut self,
        node: SharedBox<'a, TraversableJSXFragment<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_jsx_fragment(
        &mut self,
        node: SharedBox<'a, TraversableJSXFragment<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_jsx_element_name(
        &mut self,
        node: TraversableJSXElementName<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_jsx_element_name(
        &mut self,
        node: TraversableJSXElementName<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_jsx_namespaced_name(
        &mut self,
        node: SharedBox<'a, TraversableJSXNamespacedName<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_jsx_namespaced_name(
        &mut self,
        node: SharedBox<'a, TraversableJSXNamespacedName<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_jsx_member_expression(
        &mut self,
        node: SharedBox<'a, TraversableJSXMemberExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_jsx_member_expression(
        &mut self,
        node: SharedBox<'a, TraversableJSXMemberExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_jsx_member_expression_object(
        &mut self,
        node: TraversableJSXMemberExpressionObject<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_jsx_member_expression_object(
        &mut self,
        node: TraversableJSXMemberExpressionObject<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_jsx_expression_container(
        &mut self,
        node: SharedBox<'a, TraversableJSXExpressionContainer<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_jsx_expression_container(
        &mut self,
        node: SharedBox<'a, TraversableJSXExpressionContainer<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_jsx_expression(
        &mut self,
        node: TraversableJSXExpression<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_jsx_expression(
        &mut self,
        node: TraversableJSXExpression<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn visit_jsx_empty_expression(
        &mut self,
        node: SharedBox<'a, TraversableJSXEmptyExpression>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_jsx_attribute_item(
        &mut self,
        node: TraversableJSXAttributeItem<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_jsx_attribute_item(
        &mut self,
        node: TraversableJSXAttributeItem<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_jsx_attribute(
        &mut self,
        node: SharedBox<'a, TraversableJSXAttribute<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_jsx_attribute(
        &mut self,
        node: SharedBox<'a, TraversableJSXAttribute<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_jsx_spread_attribute(
        &mut self,
        node: SharedBox<'a, TraversableJSXSpreadAttribute<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_jsx_spread_attribute(
        &mut self,
        node: SharedBox<'a, TraversableJSXSpreadAttribute<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_jsx_attribute_name(
        &mut self,
        node: TraversableJSXAttributeName<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_jsx_attribute_name(
        &mut self,
        node: TraversableJSXAttributeName<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_jsx_attribute_value(
        &mut self,
        node: TraversableJSXAttributeValue<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_jsx_attribute_value(
        &mut self,
        node: TraversableJSXAttributeValue<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn visit_jsx_identifier(
        &mut self,
        node: SharedBox<'a, TraversableJSXIdentifier<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_jsx_child(
        &mut self,
        node: TraversableJSXChild<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_jsx_child(
        &mut self,
        node: TraversableJSXChild<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_jsx_spread_child(
        &mut self,
        node: SharedBox<'a, TraversableJSXSpreadChild<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_jsx_spread_child(
        &mut self,
        node: SharedBox<'a, TraversableJSXSpreadChild<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn visit_jsx_text(
        &mut self,
        node: SharedBox<'a, TraversableJSXText<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn visit_boolean_literal(
        &mut self,
        node: SharedBox<'a, TraversableBooleanLiteral>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn visit_null_literal(
        &mut self,
        node: SharedBox<'a, TraversableNullLiteral>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn visit_numeric_literal(
        &mut self,
        node: SharedBox<'a, TraversableNumericLiteral<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn visit_big_int_literal(
        &mut self,
        node: SharedBox<'a, TraversableBigIntLiteral<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn visit_reg_exp_literal(
        &mut self,
        node: SharedBox<'a, TraversableRegExpLiteral<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn visit_string_literal(
        &mut self,
        node: SharedBox<'a, TraversableStringLiteral<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_this_parameter(
        &mut self,
        node: SharedBox<'a, TraversableTSThisParameter<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_this_parameter(
        &mut self,
        node: SharedBox<'a, TraversableTSThisParameter<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_enum_declaration(
        &mut self,
        node: SharedBox<'a, TraversableTSEnumDeclaration<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_enum_declaration(
        &mut self,
        node: SharedBox<'a, TraversableTSEnumDeclaration<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_enum_member(
        &mut self,
        node: SharedBox<'a, TraversableTSEnumMember<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_enum_member(
        &mut self,
        node: SharedBox<'a, TraversableTSEnumMember<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_enum_member_name(
        &mut self,
        node: TraversableTSEnumMemberName<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_enum_member_name(
        &mut self,
        node: TraversableTSEnumMemberName<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_type_annotation(
        &mut self,
        node: SharedBox<'a, TraversableTSTypeAnnotation<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_type_annotation(
        &mut self,
        node: SharedBox<'a, TraversableTSTypeAnnotation<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_literal_type(
        &mut self,
        node: SharedBox<'a, TraversableTSLiteralType<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_literal_type(
        &mut self,
        node: SharedBox<'a, TraversableTSLiteralType<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_literal(
        &mut self,
        node: TraversableTSLiteral<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_literal(
        &mut self,
        node: TraversableTSLiteral<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_type(
        &mut self,
        node: TraversableTSType<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_type(&mut self, node: TraversableTSType<'a>, ctx: &TraverseCtx<'a>, tk: &mut Token) {
    }

    fn enter_ts_conditional_type(
        &mut self,
        node: SharedBox<'a, TraversableTSConditionalType<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_conditional_type(
        &mut self,
        node: SharedBox<'a, TraversableTSConditionalType<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_union_type(
        &mut self,
        node: SharedBox<'a, TraversableTSUnionType<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_union_type(
        &mut self,
        node: SharedBox<'a, TraversableTSUnionType<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_intersection_type(
        &mut self,
        node: SharedBox<'a, TraversableTSIntersectionType<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_intersection_type(
        &mut self,
        node: SharedBox<'a, TraversableTSIntersectionType<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_type_operator(
        &mut self,
        node: SharedBox<'a, TraversableTSTypeOperator<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_type_operator(
        &mut self,
        node: SharedBox<'a, TraversableTSTypeOperator<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_array_type(
        &mut self,
        node: SharedBox<'a, TraversableTSArrayType<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_array_type(
        &mut self,
        node: SharedBox<'a, TraversableTSArrayType<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_indexed_access_type(
        &mut self,
        node: SharedBox<'a, TraversableTSIndexedAccessType<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_indexed_access_type(
        &mut self,
        node: SharedBox<'a, TraversableTSIndexedAccessType<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_tuple_type(
        &mut self,
        node: SharedBox<'a, TraversableTSTupleType<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_tuple_type(
        &mut self,
        node: SharedBox<'a, TraversableTSTupleType<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_named_tuple_member(
        &mut self,
        node: SharedBox<'a, TraversableTSNamedTupleMember<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_named_tuple_member(
        &mut self,
        node: SharedBox<'a, TraversableTSNamedTupleMember<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_optional_type(
        &mut self,
        node: SharedBox<'a, TraversableTSOptionalType<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_optional_type(
        &mut self,
        node: SharedBox<'a, TraversableTSOptionalType<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_rest_type(
        &mut self,
        node: SharedBox<'a, TraversableTSRestType<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_rest_type(
        &mut self,
        node: SharedBox<'a, TraversableTSRestType<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_tuple_element(
        &mut self,
        node: TraversableTSTupleElement<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_tuple_element(
        &mut self,
        node: TraversableTSTupleElement<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn visit_ts_any_keyword(
        &mut self,
        node: SharedBox<'a, TraversableTSAnyKeyword>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn visit_ts_string_keyword(
        &mut self,
        node: SharedBox<'a, TraversableTSStringKeyword>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn visit_ts_boolean_keyword(
        &mut self,
        node: SharedBox<'a, TraversableTSBooleanKeyword>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn visit_ts_number_keyword(
        &mut self,
        node: SharedBox<'a, TraversableTSNumberKeyword>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn visit_ts_never_keyword(
        &mut self,
        node: SharedBox<'a, TraversableTSNeverKeyword>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn visit_ts_unknown_keyword(
        &mut self,
        node: SharedBox<'a, TraversableTSUnknownKeyword>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn visit_ts_null_keyword(
        &mut self,
        node: SharedBox<'a, TraversableTSNullKeyword>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn visit_ts_undefined_keyword(
        &mut self,
        node: SharedBox<'a, TraversableTSUndefinedKeyword>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn visit_ts_void_keyword(
        &mut self,
        node: SharedBox<'a, TraversableTSVoidKeyword>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn visit_ts_symbol_keyword(
        &mut self,
        node: SharedBox<'a, TraversableTSSymbolKeyword>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn visit_ts_this_type(
        &mut self,
        node: SharedBox<'a, TraversableTSThisType>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn visit_ts_object_keyword(
        &mut self,
        node: SharedBox<'a, TraversableTSObjectKeyword>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn visit_ts_big_int_keyword(
        &mut self,
        node: SharedBox<'a, TraversableTSBigIntKeyword>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_type_reference(
        &mut self,
        node: SharedBox<'a, TraversableTSTypeReference<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_type_reference(
        &mut self,
        node: SharedBox<'a, TraversableTSTypeReference<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_type_name(
        &mut self,
        node: TraversableTSTypeName<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_type_name(
        &mut self,
        node: TraversableTSTypeName<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_qualified_name(
        &mut self,
        node: SharedBox<'a, TraversableTSQualifiedName<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_qualified_name(
        &mut self,
        node: SharedBox<'a, TraversableTSQualifiedName<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_type_parameter_instantiation(
        &mut self,
        node: SharedBox<'a, TraversableTSTypeParameterInstantiation<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_type_parameter_instantiation(
        &mut self,
        node: SharedBox<'a, TraversableTSTypeParameterInstantiation<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_type_parameter(
        &mut self,
        node: SharedBox<'a, TraversableTSTypeParameter<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_type_parameter(
        &mut self,
        node: SharedBox<'a, TraversableTSTypeParameter<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_type_parameter_declaration(
        &mut self,
        node: SharedBox<'a, TraversableTSTypeParameterDeclaration<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_type_parameter_declaration(
        &mut self,
        node: SharedBox<'a, TraversableTSTypeParameterDeclaration<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_type_alias_declaration(
        &mut self,
        node: SharedBox<'a, TraversableTSTypeAliasDeclaration<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_type_alias_declaration(
        &mut self,
        node: SharedBox<'a, TraversableTSTypeAliasDeclaration<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_class_implements(
        &mut self,
        node: SharedBox<'a, TraversableTSClassImplements<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_class_implements(
        &mut self,
        node: SharedBox<'a, TraversableTSClassImplements<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_interface_declaration(
        &mut self,
        node: SharedBox<'a, TraversableTSInterfaceDeclaration<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_interface_declaration(
        &mut self,
        node: SharedBox<'a, TraversableTSInterfaceDeclaration<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_interface_body(
        &mut self,
        node: SharedBox<'a, TraversableTSInterfaceBody<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_interface_body(
        &mut self,
        node: SharedBox<'a, TraversableTSInterfaceBody<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_property_signature(
        &mut self,
        node: SharedBox<'a, TraversableTSPropertySignature<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_property_signature(
        &mut self,
        node: SharedBox<'a, TraversableTSPropertySignature<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_signature(
        &mut self,
        node: TraversableTSSignature<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_signature(
        &mut self,
        node: TraversableTSSignature<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_index_signature(
        &mut self,
        node: SharedBox<'a, TraversableTSIndexSignature<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_index_signature(
        &mut self,
        node: SharedBox<'a, TraversableTSIndexSignature<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_call_signature_declaration(
        &mut self,
        node: SharedBox<'a, TraversableTSCallSignatureDeclaration<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_call_signature_declaration(
        &mut self,
        node: SharedBox<'a, TraversableTSCallSignatureDeclaration<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_method_signature(
        &mut self,
        node: SharedBox<'a, TraversableTSMethodSignature<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_method_signature(
        &mut self,
        node: SharedBox<'a, TraversableTSMethodSignature<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_construct_signature_declaration(
        &mut self,
        node: SharedBox<'a, TraversableTSConstructSignatureDeclaration<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_construct_signature_declaration(
        &mut self,
        node: SharedBox<'a, TraversableTSConstructSignatureDeclaration<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_index_signature_name(
        &mut self,
        node: SharedBox<'a, TraversableTSIndexSignatureName<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_index_signature_name(
        &mut self,
        node: SharedBox<'a, TraversableTSIndexSignatureName<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_interface_heritage(
        &mut self,
        node: SharedBox<'a, TraversableTSInterfaceHeritage<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_interface_heritage(
        &mut self,
        node: SharedBox<'a, TraversableTSInterfaceHeritage<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_type_predicate(
        &mut self,
        node: SharedBox<'a, TraversableTSTypePredicate<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_type_predicate(
        &mut self,
        node: SharedBox<'a, TraversableTSTypePredicate<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_type_predicate_name(
        &mut self,
        node: TraversableTSTypePredicateName<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_type_predicate_name(
        &mut self,
        node: TraversableTSTypePredicateName<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_module_declaration(
        &mut self,
        node: SharedBox<'a, TraversableTSModuleDeclaration<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_module_declaration(
        &mut self,
        node: SharedBox<'a, TraversableTSModuleDeclaration<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_module_declaration_name(
        &mut self,
        node: TraversableTSModuleDeclarationName<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_module_declaration_name(
        &mut self,
        node: TraversableTSModuleDeclarationName<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_module_declaration_body(
        &mut self,
        node: TraversableTSModuleDeclarationBody<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_module_declaration_body(
        &mut self,
        node: TraversableTSModuleDeclarationBody<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_module_block(
        &mut self,
        node: SharedBox<'a, TraversableTSModuleBlock<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_module_block(
        &mut self,
        node: SharedBox<'a, TraversableTSModuleBlock<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_type_literal(
        &mut self,
        node: SharedBox<'a, TraversableTSTypeLiteral<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_type_literal(
        &mut self,
        node: SharedBox<'a, TraversableTSTypeLiteral<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_infer_type(
        &mut self,
        node: SharedBox<'a, TraversableTSInferType<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_infer_type(
        &mut self,
        node: SharedBox<'a, TraversableTSInferType<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_type_query(
        &mut self,
        node: SharedBox<'a, TraversableTSTypeQuery<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_type_query(
        &mut self,
        node: SharedBox<'a, TraversableTSTypeQuery<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_type_query_expr_name(
        &mut self,
        node: TraversableTSTypeQueryExprName<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_type_query_expr_name(
        &mut self,
        node: TraversableTSTypeQueryExprName<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_import_type(
        &mut self,
        node: SharedBox<'a, TraversableTSImportType<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_import_type(
        &mut self,
        node: SharedBox<'a, TraversableTSImportType<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_import_attributes(
        &mut self,
        node: SharedBox<'a, TraversableTSImportAttributes<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_import_attributes(
        &mut self,
        node: SharedBox<'a, TraversableTSImportAttributes<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_import_attribute(
        &mut self,
        node: SharedBox<'a, TraversableTSImportAttribute<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_import_attribute(
        &mut self,
        node: SharedBox<'a, TraversableTSImportAttribute<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_import_attribute_name(
        &mut self,
        node: TraversableTSImportAttributeName<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_import_attribute_name(
        &mut self,
        node: TraversableTSImportAttributeName<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_function_type(
        &mut self,
        node: SharedBox<'a, TraversableTSFunctionType<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_function_type(
        &mut self,
        node: SharedBox<'a, TraversableTSFunctionType<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_constructor_type(
        &mut self,
        node: SharedBox<'a, TraversableTSConstructorType<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_constructor_type(
        &mut self,
        node: SharedBox<'a, TraversableTSConstructorType<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_mapped_type(
        &mut self,
        node: SharedBox<'a, TraversableTSMappedType<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_mapped_type(
        &mut self,
        node: SharedBox<'a, TraversableTSMappedType<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_template_literal_type(
        &mut self,
        node: SharedBox<'a, TraversableTSTemplateLiteralType<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_template_literal_type(
        &mut self,
        node: SharedBox<'a, TraversableTSTemplateLiteralType<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_as_expression(
        &mut self,
        node: SharedBox<'a, TraversableTSAsExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_as_expression(
        &mut self,
        node: SharedBox<'a, TraversableTSAsExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_satisfies_expression(
        &mut self,
        node: SharedBox<'a, TraversableTSSatisfiesExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_satisfies_expression(
        &mut self,
        node: SharedBox<'a, TraversableTSSatisfiesExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_type_assertion(
        &mut self,
        node: SharedBox<'a, TraversableTSTypeAssertion<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_type_assertion(
        &mut self,
        node: SharedBox<'a, TraversableTSTypeAssertion<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_import_equals_declaration(
        &mut self,
        node: SharedBox<'a, TraversableTSImportEqualsDeclaration<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_import_equals_declaration(
        &mut self,
        node: SharedBox<'a, TraversableTSImportEqualsDeclaration<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_module_reference(
        &mut self,
        node: TraversableTSModuleReference<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_module_reference(
        &mut self,
        node: TraversableTSModuleReference<'a>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_external_module_reference(
        &mut self,
        node: SharedBox<'a, TraversableTSExternalModuleReference<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_external_module_reference(
        &mut self,
        node: SharedBox<'a, TraversableTSExternalModuleReference<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_non_null_expression(
        &mut self,
        node: SharedBox<'a, TraversableTSNonNullExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_non_null_expression(
        &mut self,
        node: SharedBox<'a, TraversableTSNonNullExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_decorator(
        &mut self,
        node: SharedBox<'a, TraversableDecorator<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_decorator(
        &mut self,
        node: SharedBox<'a, TraversableDecorator<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_export_assignment(
        &mut self,
        node: SharedBox<'a, TraversableTSExportAssignment<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_export_assignment(
        &mut self,
        node: SharedBox<'a, TraversableTSExportAssignment<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn visit_ts_namespace_export_declaration(
        &mut self,
        node: SharedBox<'a, TraversableTSNamespaceExportDeclaration<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_ts_instantiation_expression(
        &mut self,
        node: SharedBox<'a, TraversableTSInstantiationExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_ts_instantiation_expression(
        &mut self,
        node: SharedBox<'a, TraversableTSInstantiationExpression<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn enter_js_doc_nullable_type(
        &mut self,
        node: SharedBox<'a, TraversableJSDocNullableType<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
    fn exit_js_doc_nullable_type(
        &mut self,
        node: SharedBox<'a, TraversableJSDocNullableType<'a>>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }

    fn visit_js_doc_unknown_type(
        &mut self,
        node: SharedBox<'a, TraversableJSDocUnknownType>,
        ctx: &TraverseCtx<'a>,
        tk: &mut Token,
    ) {
    }
}
