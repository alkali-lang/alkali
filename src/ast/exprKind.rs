use super::{ArithmeticOp, Block, Expr, TypedefField};

pub enum ExprKind {
	Binary(ArithmeticOp, Box<Expr>, Box<Expr>),
	FnInvoke(Box<Expr>, Vec<Expr>),
	Typedef(Vec<TypedefField>),
	While(Block),
}
