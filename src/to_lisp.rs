use crate::gnode::Gnode;

pub trait ToLisp {
    fn to_lisp(&self, gnode: &Gnode) -> String;
}

pub fn to_lisp<T: ToLisp>(x: &T, gnode: &Gnode) -> String {
    return x.to_lisp(gnode);
}