use crate::lex::TokenKind;

use super::ExprKind;

pub struct Expr {
	pub kind: ExprKind,
	pub tokens: Vec<TokenKind>,
}
