use std::ops::Index;

use crate::TypeClass;

pub type BlockOf<T> = <T as GAllocControlFlowGraph>::Block;
pub type BlockHandleOf<T> = <BlockOf<T> as GAllocBlock>::Handle;
pub type InstructionTypeOf<T> = <BlockOf<T> as GAllocBlock>::Instruction;
pub type VariableTypeOf<T> = <InstructionTypeOf<T> as GAllocInstruction>::Variable;

/// Provides an abstraction over a CFG Block
pub trait GAllocBlock {
    type Instruction: GAllocInstruction;
    type InstructionIter<'a>: Iterator<Item = &'a Self::Instruction>
    where
        Self: 'a;
    type InstructionList: Index<usize>;
    type Handle: Copy + Clone;

    /// Returns an iterator over the instructions
    fn instructions_iter<'a>(&'a self) -> Self::InstructionIter<'a>;
    /// Returns and indexable list over the instructions
    fn instructions(&self) -> &Self::InstructionList;
}

/// GAlloc specific abstraction over a control flow graph (CFG)
pub trait GAllocControlFlowGraph {
    type Block: GAllocBlock;
    type EdgesIterator: Iterator<Item = BlockHandleOf<Self>>;

    /// Get a block refernce from a block handle
    fn block_get(&self, handle: BlockHandleOf<Self>) -> &Self::Block;
    /// Get the edges of a block from a block handle
    fn block_edges(&self, handle: BlockHandleOf<Self>) -> Self::EdgesIterator;
}

/// GAlloc specific abstraction over a variable
pub trait GAllocVariable: PartialEq {
    fn type_class(&self) -> TypeClass;
}

/// GAlloc specific abstraction over a instruction
pub trait GAllocInstruction {
    type Variable: GAllocVariable;

    /// Returns the variables read by this instruction, not necessarily those that are written to
    fn variables_read_by_instruction(&self) -> Vec<&Self::Variable>;
}
