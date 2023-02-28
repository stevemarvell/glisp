use std::fmt;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Gnode {
    pub id: usize,
    pub node_type: NodeType,
    pub children: Vec<&'static Gnode>,
    pub parent_count: usize,
}

impl Gnode {
    pub fn new(id: usize, node_type: NodeType) -> Self {
        Self {
            id,
            node_type,
            children: Vec::new(),
            parent_count: 0,
        }
    }
}

impl Gnode {
    pub fn add_links(&mut self, to: &[&mut Gnode]) {
        for child in to {
            self.children.push(child);
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
    fn to_string(&self) -> String {
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
        write!(f, "{}", self.to_string())
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
    fn to_string(&self) -> String {
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
        write!(f, "{}", self.to_string())
    }
}

impl Gnode {
    pub fn evaluate(&mut self) -> i64 {
        match self.node_type {
            NodeType::N(n) => n,
            NodeType::Math(op) => {
                let [&child1, &child2] = self.children.as_slice()
                    else {
                        panic!("Invalid number of operands for Math node");
                    };

                let left = child1.evaluate();
                let right = child2.evaluate();
                match op {
                    MathOp::Add => left + right,
                    MathOp::Sub => left - right,
                    MathOp::Mul => left * right,
                    MathOp::Div => left / right,
                }
            }
            NodeType::Cmp(op) => {
                let [child1, child2] = self.children.as_slice()
                    else {
                        panic!("Invalid number of operands for Cmp node");
                    };
                let left = child1.evaluate();
                let right = child2.evaluate();
                let result = match op {
                    CmpOp::Lt => left < right,
                    CmpOp::Le => left <= right,
                    CmpOp::Eq => left == right,
                    CmpOp::Ge => left >= right,
                    CmpOp::Gt => left > right,
                    CmpOp::Ne => left != right,
                } as i64;
                result
            }
            NodeType::Cond => {
                let [cond, then_node, else_node] = self.children.as_slice()
                    else {
                        panic!("Invalid number of operands for Cond node");
                    };
                if cond.evaluate() != 0 {
                    then_node.evaluate()
                } else {
                    else_node.evaluate()
                }
            }
        }
    }
}