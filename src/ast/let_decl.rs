use super::Expr;

#[derive(Debug, PartialEq)]
pub struct LetDecl {
	pub name: String,
	pub value: Box<Expr>,
}
