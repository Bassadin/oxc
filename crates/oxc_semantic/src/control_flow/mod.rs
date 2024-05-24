mod builder;

use oxc_span::CompactStr;
use oxc_syntax::operator::{
    AssignmentOperator, BinaryOperator, LogicalOperator, UnaryOperator, UpdateOperator,
};
use petgraph::{stable_graph::NodeIndex, Graph};

use crate::AstNodeId;

pub use builder::ControlFlowGraphBuilder;

pub type BasicBlockId = NodeIndex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Register {
    Index(u32),
    Return,
}

#[derive(Debug, Clone)]
pub enum ObjectPropertyAccessBy {
    PrivateProperty(CompactStr),
    Property(CompactStr),
    Expression(Register),
}

#[derive(Debug, Clone)]
pub struct CollectionAssignmentValue {
    pub id: AstNodeId,
    pub elements: Vec<Register>,
    pub spreads: Vec<usize>,
    pub collection_type: CollectionType,
}

#[derive(Debug, Clone)]
pub struct CalleeWithArgumentsAssignmentValue {
    pub id: AstNodeId,
    pub callee: Register,
    pub arguments: Vec<Register>,
    pub spreads: Vec<usize>,
    pub call_type: CallType,
}

#[derive(Debug, Clone)]
pub struct ObjectPropertyAccessAssignmentValue {
    pub id: AstNodeId,
    pub access_on: Register,
    pub access_by: ObjectPropertyAccessBy,
    pub optional: bool,
}

#[derive(Debug, Clone)]
pub struct BinaryAssignmentValue {
    pub id: AstNodeId,
    pub a: Register,
    pub b: Register,
    pub operator: BinaryOp,
}

#[derive(Debug, Clone)]
pub struct UpdateAssignmentValue {
    pub id: AstNodeId,
    pub expr: Register,
    pub op: UpdateOperator,
    pub prefix: bool,
}

#[derive(Debug, Clone)]
pub struct UnaryExpressioneAssignmentValue(pub AstNodeId, pub UnaryOperator, pub Register);

#[derive(Debug, Clone)]
pub enum AssignmentValue {
    ImplicitUndefined,
    NotImplicitUndefined,
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    BinaryOperator(BinaryOperator),
    LogicalOperator(LogicalOperator),
    AssignmentOperator(AssignmentOperator),
}

#[derive(Debug, Clone)]
pub enum CollectionType {
    Array,
    // Note: we do not currently track object names in objects.
    Object,
    JSXElement,
    JSXFragment,
    // doesn't use spreads
    Class,
    TemplateLiteral,
}

#[derive(Debug, Clone)]
pub enum CallType {
    New,
    CallExpression,
    // the callee is the yielded value, arguments are always empty
    // spreads are always empty
    Yield,
    // spreads are always empty
    TaggedTemplate,
    // spreads are always empty
    Import,
}

#[derive(Debug)]
pub struct BasicBlock {
    pub instructions: Vec<Instruction>,
}

impl BasicBlock {
    fn new() -> Self {
        BasicBlock { instructions: Vec::new() }
    }

    pub fn instructions(&self) -> &Vec<Instruction> {
        &self.instructions
    }
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub kind: InstructionKind,
    pub node_id: Option<AstNodeId>,
}

impl Instruction {
    pub fn new(kind: InstructionKind, node_id: Option<AstNodeId>) -> Self {
        Self { kind, node_id }
    }
}

#[derive(Debug, Clone)]
pub enum InstructionKind {
    Unreachable,
    Statement,
    Jump { conditional: bool, block: BasicBlockId },
    Return(ReturnInstructionKind),
    Throw,
    Break,
}

#[derive(Debug, Clone)]
pub enum ReturnInstructionKind {
    ImplicitUndefined,
    NotImplicitUndefined,
}

#[derive(Debug, Clone)]
pub enum BasicBlockElement {
    Unreachable,
    Assignment(Register, AssignmentValue),
    Throw(Register),
    Break(Option<Register>),
}

#[derive(Debug, Clone)]
pub enum EdgeType {
    Normal,
    Backedge,
    NewFunction,
}

#[derive(Debug)]
pub struct ControlFlowGraph {
    pub graph: Graph<usize, EdgeType>,
    pub basic_blocks: Vec<BasicBlock>,
}

impl ControlFlowGraph {
    /// # Panics
    pub fn basic_block(&self, id: BasicBlockId) -> &BasicBlock {
        let ix = *self.graph.node_weight(id).expect("expected a valid node id in self.graph");
        self.basic_blocks.get(ix).expect("expected a valid node id in self.basic_blocks")
    }

    /// # Panics
    pub fn basic_block_mut(&mut self, id: BasicBlockId) -> &mut BasicBlock {
        let ix = *self.graph.node_weight(id).expect("expected a valid node id in self.graph");
        self.basic_blocks.get_mut(ix).expect("expected a valid node id in self.basic_blocks")
    }
}

pub enum StatementControlFlowType {
    DoesNotUseContinue,
    UsesContinue,
}

pub struct PreservedStatementState {
    put_label: bool,
}

pub struct PreservedExpressionState {
    pub use_this_register: Option<Register>,
    pub store_final_assignments_into_this_array: Vec<Vec<Register>>,
}

#[must_use]
pub fn print_basic_block(basic_block: &BasicBlock) -> String {
    let mut output = String::new();
    for instruction in basic_block.instructions() {
        match instruction.kind {
            InstructionKind::Statement => output.push_str("statement\n"),
            InstructionKind::Unreachable => output.push_str("Unreachable()\n"),
            InstructionKind::Throw => output.push_str("throw\n"),
            InstructionKind::Break => output.push_str("break\n"),
            InstructionKind::Return(ReturnInstructionKind::ImplicitUndefined) => {
                output.push_str("return <implicit undefined>\n");
            }
            InstructionKind::Return(ReturnInstructionKind::NotImplicitUndefined) => {
                output.push_str("return <value>\n");
            }
            #[allow(clippy::todo)]
            InstructionKind::Jump { .. } => {
                todo!("We haven't switched to use jumps yet");
            }
        }
    }
    output
}
