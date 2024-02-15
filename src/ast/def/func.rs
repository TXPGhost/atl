use crate::ast::{expr::Expr, ty::Ty};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Func {
    argument: Ty,
    body: Box<Expr>,
}
