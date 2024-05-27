use petgraph::dot::{Config, Dot};

use crate::{
    AstNodes, BasicBlock, ControlFlowGraph, EdgeType, Instruction, InstructionKind,
    ReturnInstructionKind,
};

pub trait DisplayDot {
    fn display_dot(&self) -> String;
}

pub trait DebugDot {
    fn debug_dot(&self, ctx: DebugDotContext) -> String;
}

#[derive(Clone, Copy)]
pub struct DebugDotContext<'a, 'b> {
    pub nodes: &'b AstNodes<'a>,
}

impl DisplayDot for ControlFlowGraph {
    fn display_dot(&self) -> String {
        format!(
            "{:?}",
            Dot::with_attr_getters(
                &self.graph,
                &[Config::EdgeNoLabel, Config::NodeNoLabel],
                &|_graph, edge| {
                    let weight = edge.weight();
                    let label = format!("label = {weight:?} ");
                    if matches!(weight, EdgeType::Unreachable) {
                        format!("{label}, style = \"dotted\" ")
                    } else {
                        label
                    }
                },
                &|_graph, node| format!(
                    "label = {:?} ",
                    self.basic_blocks[*node.1].display_dot().trim()
                ),
            )
        )
    }
}

impl DisplayDot for BasicBlock {
    fn display_dot(&self) -> String {
        self.instructions().iter().fold(String::new(), |mut acc, it| {
            acc.push_str(it.display_dot().as_str());
            acc.push('\n');
            acc
        })
    }
}

impl DisplayDot for Instruction {
    fn display_dot(&self) -> String {
        match self.kind {
            InstructionKind::Statement => "statement",
            InstructionKind::Unreachable => "unreachable",
            InstructionKind::Throw => "throw",
            InstructionKind::Break => "break",
            InstructionKind::Return(ReturnInstructionKind::ImplicitUndefined) => {
                "return <implicit undefined>"
            }
            InstructionKind::Return(ReturnInstructionKind::NotImplicitUndefined) => {
                "return <value>"
            }
            #[allow(clippy::todo)]
            InstructionKind::Jump { .. } => {
                todo!("We haven't switched to use jumps yet");
            }
        }
        .to_string()
    }
}

impl DebugDot for BasicBlock {
    fn debug_dot(&self, ctx: DebugDotContext) -> String {
        self.instructions().iter().fold(String::new(), |mut acc, it| {
            acc.push_str(it.debug_dot(ctx).as_str());
            acc.push('\n');
            acc
        })
    }
}

impl DebugDot for Instruction {
    fn debug_dot(&self, _: DebugDotContext) -> String {
        match self.kind {
            InstructionKind::Statement => "statement",
            InstructionKind::Unreachable => "unreachable",
            InstructionKind::Throw => "throw",
            InstructionKind::Break => "break",
            InstructionKind::Return(ReturnInstructionKind::ImplicitUndefined) => {
                "return <implicit undefined>"
            }
            InstructionKind::Return(ReturnInstructionKind::NotImplicitUndefined) => {
                "return <value>"
            }
            #[allow(clippy::todo)]
            InstructionKind::Jump { .. } => {
                todo!("We haven't switched to use jumps yet");
            }
        }
        .to_string()
    }
}
