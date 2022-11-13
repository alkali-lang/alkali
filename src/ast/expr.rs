use crate::lex::Token;

use super::ExprKind;

#[derive(Debug)]
pub struct Expr {
	pub kind: ExprKind,
}
