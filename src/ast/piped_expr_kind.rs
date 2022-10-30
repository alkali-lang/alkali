use super::{expr::Expr, pipe_expr::PipeExpr};

pub enum PipedExprKind {
	Expr(Expr),
	PipeExpr(PipeExpr),
}
