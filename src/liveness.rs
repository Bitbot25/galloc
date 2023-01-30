use crate::cfg::{self, GAllocBlock, GAllocInstruction};

/// A location in the program
///
/// The location features a block and instruction index that is relative to the block.
#[derive(Debug)]
pub struct Location<H> {
    /// The handle to the block
    block: H,
    /// The instruction index relative to the block
    block_relative_index: usize,
}

impl<H> Location<H> {
    /// Returns the block handle (H) of this [`Location`]
    #[inline]
    pub fn block_handle(&self) -> &H {
        &self.block
    }

    /// The instruction index relative to this [`Location::block_handle`]
    #[inline]
    pub fn block_relative_index(&self) -> usize {
        self.block_relative_index
    }
}

/// Search for deaths of the variable in each ending branch of the program.
///
/// # Return value
/// The return value is a [`Vec`], containing all the [`Location`]s where the variable dies (is no longer live).
///
/// # Panics
/// This function will never panic
#[inline]
pub fn search_final_branch_uses<CFG>(
    var: &cfg::VariableTypeOf<CFG>,
    var_definition_block: cfg::BlockHandleOf<CFG>,
    graph: &CFG,
) -> Vec<Location<cfg::BlockHandleOf<CFG>>>
where
    CFG: cfg::GAllocControlFlowGraph,
{
    let mut final_uses = Vec::new();
    internal_search_final_branch_uses(var, graph, var_definition_block, 0, &mut final_uses);
    final_uses
}

/// Internal implementation of [`search_final_branch_uses`]
fn internal_search_final_branch_uses<CFG>(
    var: &cfg::VariableTypeOf<CFG>,
    graph: &CFG,
    current_block: cfg::BlockHandleOf<CFG>,
    mut branch_id: usize,
    final_uses: &mut Vec<Location<cfg::BlockHandleOf<CFG>>>,
) where
    CFG: cfg::GAllocControlFlowGraph,
{
    fn insert_use<CFG: cfg::GAllocControlFlowGraph>(
        list: &mut Vec<Location<cfg::BlockHandleOf<CFG>>>,
        branch_id: usize,
        location: Location<cfg::BlockHandleOf<CFG>>,
    ) {
        if branch_id >= list.len() {
            assert_eq!(
                branch_id,
                list.len(),
                "Locations can only be expanded one at a time."
            );
            list.push(location);
        } else {
            list[branch_id] = location;
        }
    }

    for (ins_index, ins) in graph
        .block_get(current_block)
        .instructions_iter()
        .enumerate()
    {
        if ins.variables_read_by_instruction().contains(&var) {
            insert_use::<CFG>(
                final_uses,
                branch_id,
                Location {
                    block: current_block,
                    block_relative_index: ins_index,
                },
            );
        }
    }
    for descendant in graph.block_edges(current_block) {
        internal_search_final_branch_uses(var, graph, descendant, branch_id, final_uses);
        branch_id += 1;
    }
}

/*
definition = get_definition(var)
deaths = search_final_branch_uses(var, definition) # Get the last uses for the variable in each branch
for death in deaths:
    mark_live_before(var, definition, death, death.block)

function mark_live_before(var, definition, death, current_block):
    # FIXME: Handle when definition and death block are the same
    if current_block is definition.block:
        for ins in current_block.instructions[definition.block_relative_index..]
            mark_live_at_instruction(var, ins)
        return
    if current_block is death.block:
        for ins in current_block.instructions[0..death.block_relative_index]:
            mark_live_at_instruction(var, ins);
    for predecessor in current_block.predecessors:
        mark_live_before(var, definition, death, predecessor)

function search_final_branch_uses(var, current_block):
    return search_final_branch_uses(var, current_block, 0)

function search_final_branch_uses(var, current_block, branch_id):
    final_uses = []
    for ins in current_block:
        if ins.variables_that_are_read() contains var:
            final_uses[branch_id] = location { block=current_block, block_relative_index=ins.index }
    for descendant in current_block.descendants:
        search_final_branch_uses(var, descendant, branch_id)
        branch_id += 1

*/
