use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone)]
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
	Unknown,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
	pub kind: TokenKind,
	pub row: usize,
	pub col: usize,
}

pub struct TokenReader {
	pub tokens: Vec<Token>,
	pub position: usize,
}

impl TokenReader {
	pub fn new(tokens: Vec<Token>) -> TokenReader {
		TokenReader {
			tokens,
			position: 0,
		}
	}

	pub fn peek(&self) -> Token {
		self.tokens.get(self.position).cloned().unwrap()
	}

	pub fn next_tokens(&mut self, n: usize) -> Token {
		self.position += n;
		self.next_token()
	}

	pub fn end_of_file(&self) -> bool {
		self.peek().kind == TokenKind::End
	}

	pub fn next_token(&mut self) -> Token {
		self.position += 1;
		let token = self.peek();
		token
	}
}
