use std::collections::HashSet;

use cfg::GAllocVariable;

pub mod cfg;
pub mod liveness;

/// A range of instructions in a program
pub struct ProgramRange {
    /// Represents the start index (inclusive)
    pub begin: usize,
    /// Represents the end index (exclusive)
    pub end: usize,
}

impl ProgramRange {
    /// Checks if this [`ProgramRange`] intersects with [`other`], e.g if they contain some common index.
    #[inline]
    pub fn interferes_with(&self, other: &ProgramRange) -> bool {
        let begin_interferes = self.begin >= other.begin && self.begin < other.end;
        let end_interferes = self.end > other.begin && self.end <= other.end;
        begin_interferes || end_interferes
    }
}

impl std::ops::RangeBounds<usize> for ProgramRange {
    #[inline]
    fn start_bound(&self) -> std::ops::Bound<&usize> {
        std::ops::Bound::Included(&self.begin)
    }

    #[inline]
    fn end_bound(&self) -> std::ops::Bound<&usize> {
        std::ops::Bound::Excluded(&self.end)
    }
}

/// Represents multiple regions where a variable is currently live in the program
pub struct Liveness {
    /// All the instruction indexes of when the variable is live.
    instructions: Vec<usize>,
}

impl Liveness {
    /// Checks if both ranges are live at the same time
    pub fn interferes_with(&self, other: &Liveness) -> bool {
        let mut set = HashSet::new();
        for instruction in &self.instructions {
            set.insert(*instruction);
        }

        for instruction in &other.instructions {
            if set.contains(instruction) {
                return true;
            }
        }

        false
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum TypeClass {
    Int8,
    Int16,
    Int32,
    Int64,
    Huge,
}

#[derive(Debug, Clone, Copy)]
pub struct IFRNodeHandle {
    index: usize,
}

#[derive(Debug)]
pub struct IFREdge {
    a: IFRNodeHandle,
    b: IFRNodeHandle,
}

/// Variable Interference Graph
#[derive(Debug)]
pub struct IFRGraph<V: GAllocVariable> {
    /// All nodes of this graph
    nodes: Vec<V>,
    edges: Vec<IFREdge>,
}

impl<V: GAllocVariable> IFRGraph<V> {
    /// Create a new empty interference graph
    #[inline]
    pub fn new() -> Self {
        IFRGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    /// Push a node onto this interference graph
    #[inline]
    pub fn push_node(&mut self, node: V) -> IFRNodeHandle {
        self.nodes.push(node);
        IFRNodeHandle {
            index: self.nodes.len() - 1,
        }
    }

    #[inline]
    pub fn push_edge(&mut self, a: IFRNodeHandle, b: IFRNodeHandle) {
        self.edges.push(IFREdge { a, b });
    }
}

impl<V: GAllocVariable> Default for IFRGraph<V> {
    fn default() -> Self {
        Self::new()
    }
}
