use super::def::Def;

pub mod block;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    Def(Def),
}
