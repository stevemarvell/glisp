use std::collections::HashMap;
use std::fmt;

use gnode::*;
pub use to_lisp::ToLisp;

pub mod gnode;
pub mod to_lisp;

#[derive(Debug)]
pub struct Glisp {
    nodes: Vec<Gnode>,
    next_id: usize,
}

impl Glisp {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            next_id: 0,
        }
    }
}

impl Glisp {
    pub fn add_node<T: Into<NodeType>>(&mut self, node_type: T) -> &mut Gnode {
        let id = self.next_id;
        let nt = node_type.into();
        let gnode = Gnode::new(id, nt);
        self.nodes.insert(id, gnode);
        self.next_id += 1;
        self.nodes.get_mut(id).unwrap()
    }
}


impl Glisp {
    pub fn evaluate(&self, gnode: &Gnode) -> i64 {
        gnode.evaluate()
    }
}

impl Glisp {
    pub fn lisp_for(&self, gnode: &Gnode) -> String {
        gnode.to_lisp()
    }
}

impl fmt::Display for Glisp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for node in self.nodes.iter() {
            writeln!(f, "{:?}", node);
            for link in node.children {
                writeln!(f, "\t{:?}", link);
            }
        }
        Ok(())
    }
}