use super::{ArithmeticOp, Block, Expr, TypedefField};

#[derive(Debug)]
pub enum ExprKind {
	Binary(ArithmeticOp, Box<Expr>, Box<Expr>),
	FnInvoke(Box<Expr>, Vec<Expr>),
	Typedef(Vec<TypedefField>),
	While(Block),
	NumLit(f64),
	StrLit(String),
}
