use std::collections::HashMap;

use gnode::*;

pub mod gnode;

#[derive(Debug)]
pub struct Glisp {
    nodes: HashMap<Gnode, Vec<Gnode>>,
    next_id: u64,
}

impl Glisp {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn add_node<T: Into<NodeType>>(&mut self, node_type: T) -> Gnode {
        let id = self.next_id;
        let nt = node_type.into();
        let gnode = Gnode::new(id, nt);
        self.next_id += 1;
        gnode
    }

    pub fn add_link(&mut self, from: Gnode, to: &[Gnode]) {
        let nodes = self.nodes.entry(from).or_insert(Vec::new());
        nodes.extend(to.iter().cloned());
    }
}

impl Glisp {
    pub fn to_lisp(&self, gnode: &Gnode) -> String {
        match gnode.node_type {
            NodeType::N(n) => n.to_string(),
            NodeType::Math(op) => {
                if let [left, right] = self.nodes.get(gnode).unwrap().as_slice() {
                    format!("({} {} {})", op, self.to_lisp(&left), self.to_lisp(&right))
                } else {
                    panic!("Invalid number of operands for Math node");
                }
            }
            NodeType::Cmp(op) => {
                if let [left, right] = self.nodes.get(gnode).unwrap().as_slice() {
                    format!("({} {} {})", op, self.to_lisp(&left), self.to_lisp(&right))
                } else {
                    panic!("Invalid number of operands for Cmp node");
                }
            }
            NodeType::Cond => {
                if let [cond, then_link, else_link] = self.nodes.get(gnode).unwrap().as_slice() {
                    format!("(? {} {} {})", self.to_lisp(&cond), self.to_lisp(&then_link), self.to_lisp(&else_link))
                } else {
                    panic!("Invalid number of links for Cond node");
                }
            }
        }
    }
}


impl Glisp {
    pub fn evaluate(&self, gnode: &Gnode) -> i64 {
        match gnode.node_type {
            NodeType::N(n) => n,
            NodeType::Math(op) => {
                let children = self.nodes.get(gnode).unwrap();
                let child1 = self.evaluate(&children[0]);
                let child2 = self.evaluate(&children[1]);
                match op {
                    MathOp::Add => child1 + child2,
                    MathOp::Sub => child1 - child2,
                    MathOp::Mul => child1 * child2,
                    MathOp::Div => child1 / child2,
                }
            }
            NodeType::Cmp(op) => {
                let children = self.nodes.get(gnode).unwrap();
                let child1 = self.evaluate(&children[0]);
                let child2 = self.evaluate(&children[1]);
                let result = match op {
                    CmpOp::Lt => child1 < child2,
                    CmpOp::Le => child1 <= child2,
                    CmpOp::Eq => child1 == child2,
                    CmpOp::Ge => child1 >= child2,
                    CmpOp::Gt => child1 > child2,
                    CmpOp::Ne => child1 != child2,
                } as i64;
                result
            }
            NodeType::Cond => {
                let children = self.nodes.get(gnode).unwrap();
                let condition = self.evaluate(&children[0]);
                if condition != 0 {
                    self.evaluate(&children[1])
                } else {
                    self.evaluate(&children[2])
                }
            }
        }
    }
}
