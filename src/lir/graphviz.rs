use super::tree::{BasicBlock, LoweredFunction, LoweredProgram};
use graphviz_rust::dot_generator::*;
use graphviz_rust::dot_structures::*;
use graphviz_rust::attributes::*;

impl LoweredProgram {
    pub fn print(&self) -> Graph {
        let functions: Vec<_> = self
            .get_entries()
            .iter()
            .map(|x| stmt!(Self::map_function(x)))
            .collect();

        graph!(strict di id!("t"),
            functions)
    }

    fn map_function(function: &LoweredFunction) -> Subgraph {
        subgraph!(function.id.get_name().clone(), Self::map_block(&function.entry))
    }

    fn map_block(block: &BasicBlock) -> Vec<Stmt> {
        vec![stmt!(node!("aaa"; attr!("shape","square")))]
    }
}
