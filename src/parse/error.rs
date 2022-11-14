use crate::lex::TokenKind;
use std::{error::Error, fmt};

#[derive(Debug, PartialEq)]
pub struct ExpectedTokenError {
	pub expected: TokenKind,
	pub found: TokenKind,
}

impl ExpectedTokenError {
	pub fn new(expected: TokenKind, found: TokenKind) -> Self {
		Self { expected, found }
	}
}

impl Error for ExpectedTokenError {}

impl fmt::Display for ExpectedTokenError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"Expected token {:?}, found {:?}",
			self.expected, self.found
		)
	}
}
