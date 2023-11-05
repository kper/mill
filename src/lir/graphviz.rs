use crate::lir::tree::LoweredStatement;

use super::tree::{BasicBlock, LoweredFunction, LoweredProgram};
use graphviz_rust::dot_generator::*;
use graphviz_rust::dot_structures::*;

impl LoweredProgram {
    pub fn print(&self) -> Graph {
        let functions: Vec<_> = self
            .get_entries()
            .iter()
            .map(|x| stmt!(self.map_function(x)))
            .collect();

        graph!(strict di id!(self.get_name()),
            functions)
    }

    fn map_function(&self, function: &LoweredFunction) -> Subgraph {
        let mut stmts: Vec<_> = function
            .blocks
            .iter()
            .map(|x| self.map_block(x))
            .flatten()
            .collect();
        let mut edges: Vec<_> = function
            .blocks
            .iter()
            .map(|x| self.map_edges(x))
            .flatten()
            .map(|x| Stmt::Edge(x))
            .collect();

        stmts.append(&mut edges);

        subgraph!(function.id.get_name().clone(), stmts)
    }

    fn map_edges(&self, block: &BasicBlock) -> Vec<Edge> {
        let mut edges = Vec::new();
        for succ in block.get_next() {
            edges.push(
                edge!(node_id!(format!("b{}", block.get_id().get_value())) => node_id!(format!("b{}", succ.get_value())))
            );
        }
        edges
    }

    fn map_block(&self, block: &BasicBlock) -> Vec<Stmt> {
        let mut content = String::new();

        for stmt in block.get_statements() {
            match stmt {
                LoweredStatement::Definition(ident, expr) => {
                    content.push_str(
                        format!("def {} = {:?}", ident.get_ident().get_name(), expr).as_str(),
                    );
                }
                LoweredStatement::Assignment(ident, expr) => {
                    content.push_str(
                        format!("{} = {:?}", ident.get_ident().get_name(), expr).as_str(),
                    );
                }
                LoweredStatement::Conditional_Jump(condition, block) => {
                    content.push_str(
                        format!("if {:?} -> jump {:?}", condition, block.get_id()).as_str(),
                    );
                }
                LoweredStatement::RetVoid => content.push_str("return (void)"),
                _ => unimplemented!(),
            }

            content.push_str("\n");
        }

        let name = format!("b{}", block.get_id().get_value());
        let value = format!(r#""{}""#, content);

        vec![stmt!(
            node!(name; attr!("label", value), attr!("xlabel", name))
        )]
    }
}
