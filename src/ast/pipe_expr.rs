use super::Expr;

pub struct PipeExpr {
	pub left: Box<Expr>,
	pub right: Box<Expr>,
}
