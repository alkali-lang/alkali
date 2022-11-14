use super::{BinaryOp, Block, Expr, TypedefField};

#[derive(Debug, PartialEq)]
pub enum ExprKind {
	Binary(BinaryOp, Box<Expr>, Box<Expr>),
	FnInvoke(Box<Expr>, Vec<Expr>),
	Typedef(Vec<TypedefField>),
	While(Block),
	NumLit(f64),
	StrLit(String),
	Group(Box<Expr>),
	Reference(String),
	// term → factor ( ( "-" | "+" ) factor )* ;
	Term(Box<Expr>),
	// factor → unary ( ( "/" | "*" ) unary )* ;
	Factor(Box<Expr>),
}
