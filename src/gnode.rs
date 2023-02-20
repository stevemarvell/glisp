use std::fmt;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Gnode {
    pub id: u64,
    pub node_type: NodeType,
}

impl Gnode {
    pub fn new(id: u64, node_type: NodeType) -> Gnode {
        Gnode {
            id: id,
            node_type: node_type,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum NodeType {
    N(i64),
    Math(MathOp),
    Cmp(CmpOp),
    Cond,
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum MathOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl MathOp {
    pub fn to_str(&self) -> &'static str {
        match self {
            MathOp::Add => "+",
            MathOp::Sub => "-",
            MathOp::Mul => "*",
            MathOp::Div => "/",
        }
    }
}

impl std::fmt::Display for MathOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum CmpOp {
    Lt,
    Le,
    Eq,
    Ge,
    Gt,
    Ne,
}

impl CmpOp {
    pub fn to_str(&self) -> &'static str {
        match self {
            CmpOp::Lt => "<",
            CmpOp::Le => "<=",
            CmpOp::Eq => "==",
            CmpOp::Ge => ">=",
            CmpOp::Gt => ">",
            CmpOp::Ne => "!=",
        }
    }
}

impl std::fmt::Display for CmpOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

