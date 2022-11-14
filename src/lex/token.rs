use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenKind {
	Equals,
	Plus,
	Minus,
	Star,
	Slash,
	Percent,
	Caret,
	Ampersand,
	Identifier(String),
	NumberLiteral(String),
	StringLiteral(String),
	End,
	Semicolon,
	LessThan,
	GreaterThan,
	Pipe,
	LBrace,
	RBrace,
	LParen,
	RParen,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
	pub kind: TokenKind,
	pub row: usize,
	pub col: usize,
}
