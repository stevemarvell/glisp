use std::fmt;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Gnode {
    pub id: u64,
    pub node_type: NodeType,
}

impl Gnode {
    pub fn new(id: u64, node_type: NodeType) -> Self {
        Self { id, node_type }
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
    fn to_str(&self) -> String {
        match self {
            Self::Add => "+".to_string(),
            Self::Sub => "-".to_string(),
            Self::Mul => "*".to_string(),
            Self::Div => "/".to_string(),
        }
    }
}

impl fmt::Display for MathOp {
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
    fn to_str(&self) -> String {
        match self {
            Self::Lt => "<".to_string(),
            Self::Le => "<=".to_string(),
            Self::Eq => "==".to_string(),
            Self::Ge => ">=".to_string(),
            Self::Gt => ">".to_string(),
            Self::Ne => "!=".to_string(),
        }
    }
}

impl fmt::Display for CmpOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}
