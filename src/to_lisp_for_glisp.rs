//use crate::gnode::*;
use crate::{Glisp, ToLisp};
use crate::gnode::*;

impl ToLisp for Glisp {
    fn to_lisp(&self, gnode: &Gnode) -> String {
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