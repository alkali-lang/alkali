use std::fmt::Error;

use super::{Token, TokenKind};

pub struct Lexer {
	src: Vec<char>,
	pub is_eof: bool,
	pub index: usize,
	pub col: usize,
	pub row: usize,
	token: Option<Token>,
}

impl Lexer {
	/// Creates a new `Lexer` from given an input string.
	pub fn new(src: &str) -> Lexer {
		let mut lexer = Lexer {
			src: src.chars().collect(),
			is_eof: false,
			index: 0,
			col: 1,
			row: 1,
			token: None,
		};

		lexer.next().unwrap();

		lexer
	}

	/// Returns the next char in the input stream
	pub fn peek_char(&self) -> Option<&char> {
		self.src.get(self.index)
	}

	// Returns the next token in the input stream
	pub fn peek_token(&self) -> Token {
		self.token.clone().unwrap()
	}

	fn lex_number(&mut self) -> Result<Token, Error> {
		let mut word = String::new();
		let temp_col = self.col;

		while self.peek_char().is_some() && self.peek_char().unwrap().is_numeric() {
			if self.is_eof() {
				return Ok(Token {
					row: self.row,
					col: self.col,
					kind: TokenKind::End,
				});
			}

			word.push(*self.peek_char().unwrap());
			self.advance();
		}

		let token = Token {
			row: self.row,
			col: temp_col,
			kind: TokenKind::NumberLiteral(word),
		};

		self.token = Some(token.clone());
		Ok(token)
	}

	fn lex_ident(&mut self) -> Result<Token, Error> {
		let mut word = String::new();

		let tmp_col = self.col;

		while self.peek_char().is_some() && self.peek_char().unwrap().is_alphabetic() {
			if self.is_eof() {
				return Ok(Token {
					row: self.row,
					col: self.col,
					kind: TokenKind::End,
				});
			}

			word.push(*self.peek_char().unwrap());
			self.advance();
		}

		let token = Token {
			row: self.row,
			col: tmp_col,
			kind: TokenKind::Identifier(word),
		};

		self.token = Some(token.clone());
		Ok(token)
	}

	fn lex_str(&mut self) -> Token {
		let mut word = String::new();
		let tmp_col = self.col;

		while self.peek_char().is_some() && self.peek_char().unwrap() != &'"' {
			word.push(*self.peek_char().unwrap());
			self.advance();
		}

		self.advance();

		let token = Token {
			kind: TokenKind::StringLiteral(word),
			row: self.row,
			col: tmp_col,
		};

		self.token = Some(token.clone());
		token
	}

	fn is_eof(&self) -> bool {
		self.index >= self.src.len()
	}

	fn advance(&mut self) {
		if self.peek_char() == Some(&'\n') {
			self.row += 1;
			self.col = 1;
		} else {
			self.col += 1;
		}

		self.index += 1;
	}

	fn lex_symbol(&mut self) -> Token {
		let kind = match self.peek_char().unwrap() {
			'=' => TokenKind::Equals,
			'+' => TokenKind::Plus,
			'-' => TokenKind::Minus,
			'*' => TokenKind::Star,
			'/' => TokenKind::Slash,
			'%' => TokenKind::Percent,
			'^' => TokenKind::Caret,
			'&' => TokenKind::Ampersand,
			';' => TokenKind::Semicolon,
			'{' => TokenKind::LBrace,
			'}' => TokenKind::RBrace,
			'(' => TokenKind::LParen,
			')' => TokenKind::RParen,
			'<' => TokenKind::LessThan,
			_ => todo!(),
		};

		let token = Token {
			kind,
			row: self.row,
			col: self.col,
		};

		self.advance();

		self.token = Some(token.clone());
		token
	}

	/// Advances the lexer to the next token
	pub fn next(&mut self) -> Result<(), Error> {
		while self.peek_char().is_some()
			&& self.peek_char().unwrap().is_whitespace()
			&& !self.is_eof()
		{
			self.advance();
		}

		if self.is_eof() {
			self.token = Some(Token {
				kind: TokenKind::End,
				row: self.row,
				col: self.col,
			});
			return Ok(());
		}

		let res = match self.peek_char().unwrap() {
			'"' => {
				self.advance();
				Ok(self.lex_str())
			}
			'>' => {
				let tmp_col = self.col;
				self.advance();

				if self.peek_char() == Some(&'>') {
					self.advance();
					self.token = Some(Token {
						row: self.row,
						col: tmp_col,
						kind: TokenKind::Pipe,
					});
					return Ok(());
				}

				Ok(Token {
					row: self.row,
					col: tmp_col,
					kind: TokenKind::GreaterThan,
				})
			}
			alpha if alpha.is_alphabetic() => self.lex_ident(),
			num if num.is_numeric() => self.lex_number(),
			_ => Ok(self.lex_symbol()),
		}?;

		self.token = Some(res);
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn vectorize(lexer: &mut Lexer) -> Vec<Token> {
		let mut vec = Vec::new();

		while lexer.peek_token().kind != TokenKind::End {
			let token = lexer.peek_token();
			vec.push(token);
			lexer.next().unwrap();
		}

		vec
	}

	#[test]
	fn test_basic() {
		let source = "test = 1 + 278";
		let tokens = vectorize(&mut Lexer::new(source));
		assert_eq!(
			tokens,
			vec![
				Token {
					kind: TokenKind::Identifier("test".to_string()),
					row: 1,
					col: 1
				},
				Token {
					kind: TokenKind::Equals,
					row: 1,
					col: 6
				},
				Token {
					kind: TokenKind::NumberLiteral("1".to_string()),
					row: 1,
					col: 8
				},
				Token {
					kind: TokenKind::Plus,
					row: 1,
					col: 10
				},
				Token {
					kind: TokenKind::NumberLiteral("278".to_string()),
					row: 1,
					col: 12
				},
			]
		);
	}

	#[test]
	fn lex_str() {
		let source = "let x = \"hello world\"";
		let tokens = vectorize(&mut Lexer::new(source));
		assert_eq!(
			tokens,
			vec![
				Token {
					kind: TokenKind::Identifier("let".to_string()),
					row: 1,
					col: 1
				},
				Token {
					kind: TokenKind::Identifier("x".to_string()),
					row: 1,
					col: 5
				},
				Token {
					kind: TokenKind::Equals,
					row: 1,
					col: 7
				},
				Token {
					kind: TokenKind::StringLiteral("hello world".to_string()),
					row: 1,
					col: 10
				},
			]
		);
	}

	#[test]
	fn test_greater_than() {
		let source = "test = 1 > 278";
		let tokens = vectorize(&mut Lexer::new(source));
		assert_eq!(
			tokens,
			vec![
				Token {
					kind: TokenKind::Identifier("test".to_string()),
					row: 1,
					col: 1
				},
				Token {
					kind: TokenKind::Equals,
					row: 1,
					col: 6
				},
				Token {
					kind: TokenKind::NumberLiteral("1".to_string()),
					row: 1,
					col: 8
				},
				Token {
					kind: TokenKind::GreaterThan,
					row: 1,
					col: 10
				},
				Token {
					kind: TokenKind::NumberLiteral("278".to_string()),
					row: 1,
					col: 12
				},
			]
		);
	}

	#[test]
	fn test_multiline() {
		let source = "test = 1 + 278\n2 + 3";
		let tokens = vectorize(&mut Lexer::new(source));
		assert_eq!(
			tokens,
			vec![
				Token {
					kind: TokenKind::Identifier("test".to_string()),
					row: 1,
					col: 1
				},
				Token {
					kind: TokenKind::Equals,
					row: 1,
					col: 6
				},
				Token {
					kind: TokenKind::NumberLiteral("1".to_string()),
					row: 1,
					col: 8
				},
				Token {
					kind: TokenKind::Plus,
					row: 1,
					col: 10
				},
				Token {
					kind: TokenKind::NumberLiteral("278".to_string()),
					row: 1,
					col: 12
				},
				Token {
					kind: TokenKind::NumberLiteral("2".to_string()),
					row: 2,
					col: 1
				},
				Token {
					kind: TokenKind::Plus,
					row: 2,
					col: 3
				},
				Token {
					kind: TokenKind::NumberLiteral("3".to_string()),
					row: 2,
					col: 5
				},
			]
		);
	}

	#[test]
	fn pipe_test() {
		let source = "test = 1 + 278 >> 2 + 3";
		let tokens = vectorize(&mut Lexer::new(source));
		assert_eq!(
			tokens,
			vec![
				Token {
					kind: TokenKind::Identifier("test".to_string()),
					row: 1,
					col: 1
				},
				Token {
					kind: TokenKind::Equals,
					row: 1,
					col: 6
				},
				Token {
					kind: TokenKind::NumberLiteral("1".to_string()),
					row: 1,
					col: 8
				},
				Token {
					kind: TokenKind::Plus,
					row: 1,
					col: 10
				},
				Token {
					kind: TokenKind::NumberLiteral("278".to_string()),
					row: 1,
					col: 12
				},
				Token {
					kind: TokenKind::Pipe,
					row: 1,
					col: 16
				},
				Token {
					kind: TokenKind::NumberLiteral("2".to_string()),
					row: 1,
					col: 19
				},
				Token {
					kind: TokenKind::Plus,
					row: 1,
					col: 21
				},
				Token {
					kind: TokenKind::NumberLiteral("3".to_string()),
					row: 1,
					col: 23
				},
			]
		);
	}
}
