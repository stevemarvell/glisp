use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum Gnode {
    N(i64),
}

#[derive(Debug)]
pub struct Glisp {
    gnodes: HashMap<Gnode, Vec<Gnode>>,
}

impl Glisp {
    pub fn new() -> Glisp {
        Glisp { gnodes: HashMap::new() }
    }

    pub fn add_gnode(&mut self, gnode: Gnode) {
        self.gnodes.entry(gnode).or_insert(Vec::new());
    }

    pub fn add_link(&mut self, from: Gnode, to: Gnode) {
        self.gnodes.entry(from).or_insert(Vec::new()).push(to);
    }
}

