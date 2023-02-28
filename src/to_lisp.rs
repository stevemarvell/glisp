use crate::gnode::*;

pub trait ToLisp {
    fn to_lisp(&self) -> String;
}

pub fn to_lisp<T: ToLisp>(x: &T) -> String {
    x.to_lisp()
}

impl ToLisp for Gnode {
    fn to_lisp(&self) -> String {
        match self.node_type {
            NodeType::N(n) => n.to_string(),
            NodeType::Math(op) => {
                if let [left, right] = self.children.as_slice() {
                    format!("({} {} {})", op, left.to_lisp(), right.to_lisp())
                } else {
                    panic!("Invalid number of operands for Math node");
                }
            }
            NodeType::Cmp(op) => {
                if let [left, right] = self.children.as_slice() {
                    format!("({} {} {})", op, left.to_lisp(), right.to_lisp())
                } else {
                    panic!("Invalid number of operands for Cmp node");
                }
            }
            NodeType::Cond => {
                if let [cond, then_link, else_link] = self.children.as_slice() {
                    format!("(? {} {} {})", cond.to_lisp(), then_link.to_lisp(), else_link.to_lisp())
                } else {
                    panic!("Invalid number of operands for Cond node");
                }
            }
        }
    }
}
