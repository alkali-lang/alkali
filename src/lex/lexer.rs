use std::fs::read_to_string;

use super::{token, Token, TokenKind};

pub fn lex_source(source: &str) -> token::TokenReader {
	let src = read_to_string(source).expect("test");

	let tokens = Lexer::new(src.as_str()).vectorize();
	token::TokenReader::new(tokens)
}

pub struct Lexer {
	src: Vec<char>,
	pub is_eof: bool,
	pub index: usize,
	pub start: bool,
}

impl Lexer {
	pub fn new(src: &str) -> Lexer {
		Lexer {
			src: src.chars().collect(),
			is_eof: false,
			index: 0,
			start: true,
		}
	}

	fn eat_trivia(&mut self) {
		for ch in self.src.clone() {
			match ch {
				' ' | '\t' | '\r' => {
					self.next_char();
				}
				_ => (),
			};
		}
	}

	pub fn vectorize(&mut self) -> Vec<Token> {
		let mut vec = Vec::new();
		while self.index < self.src.len() {
			vec.push(self.next_token());
		}

		vec
	}

	pub fn peek_char(&self) -> Option<&char> {
		self.src.get(self.index + 1)
	}

	fn next_char(&mut self) -> char {
		self.index += 1;

		let char: Option<&char> = self.src.get(self.index);

		if let None = char {
			return '\0';
		}

		let char = char.unwrap();

		if char == &'\n' {
			return self.next_char();
		}

		char.clone()
	}

	pub fn next_token(&mut self) -> Token {
		if self.src.get(self.index).is_none() {
			return Token {
				kind: TokenKind::End,
			};
		}

		if let None = self.peek_char() {
			self.index += 1;
			return Token {
				kind: TokenKind::End,
			};
		}

		loop {
			match self.peek_char() {
				Some(&ch) => {
					if ch == ' ' || ch == '\t' || ch == '\r' {
						self.next_char();
					} else {
						break;
					}
				}
				None => break,
			}
		}

		let char = match self.start {
			true => {
				self.start = false;
				self.src.get(0).unwrap().clone()
			}
			false => self.next_char().clone(),
		};

		let kind = match char {
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
			'"' => {
				let mut char = self.next_char();
				let mut string = String::new();
				while char.is_alphanumeric() {
					string += &char.to_string();
					char = self.next_char();
				}

				TokenKind::StringLiteral(string)
			}

			pipe if pipe == '>' && self.peek_char() == Some(&'>') => {
				self.next_char();
				TokenKind::Pipe
			}
			'<' => TokenKind::LessThan,
			'>' => TokenKind::GreaterThan,
			alpha if alpha.is_alphabetic() => {
				let mut string = String::new();
				string += &alpha.to_string();

				loop {
					let char = self.peek_char();
					if let Some(&ch) = char {
						if ch.is_alphanumeric() {
							string += &ch.to_string();
							self.next_char();
						} else {
							break;
						}
					} else {
						break;
					}
				}

				TokenKind::Identifier(string)
			}
			num if num.is_numeric() => {
				let mut char = num;
				let mut number = String::new();

				loop {
					number += &char.to_string();
					let peeked_char = &self.peek_char().cloned();

					if !(peeked_char.is_some() && peeked_char.unwrap().is_numeric()) {
						break;
					}

					char = self.next_char();
				}

				TokenKind::NumberLiteral(number)
			}
			_ => TokenKind::Unknown,
		};

		Token { kind }
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_basic() {
		let source = "test = 1 + 278";
		let tokens = Lexer::new(source).vectorize();
		assert_eq!(
			tokens,
			vec![
				Token {
					kind: TokenKind::Identifier("test".to_string()),
				},
				Token {
					kind: TokenKind::Equals,
				},
				Token {
					kind: TokenKind::NumberLiteral("1".to_string()),
				},
				Token {
					kind: TokenKind::Plus,
				},
				Token {
					kind: TokenKind::NumberLiteral("278".to_string()),
				},
				Token {
					kind: TokenKind::End,
				}
			]
		);
	}

	#[test]
	fn test_multiline() {
		let source = "test = 1 + 278\n2 + 3";
		let tokens = Lexer::new(source).vectorize();
		assert_eq!(
			tokens,
			vec![
				Token {
					kind: TokenKind::Identifier("test".to_string()),
				},
				Token {
					kind: TokenKind::Equals,
				},
				Token {
					kind: TokenKind::NumberLiteral("1".to_string()),
				},
				Token {
					kind: TokenKind::Plus,
				},
				Token {
					kind: TokenKind::NumberLiteral("278".to_string()),
				},
				Token {
					kind: TokenKind::NumberLiteral("2".to_string()),
				},
				Token {
					kind: TokenKind::Plus,
				},
				Token {
					kind: TokenKind::NumberLiteral("3".to_string()),
				},
				Token {
					kind: TokenKind::End,
				}
			]
		);
	}

	#[test]
	fn pipe_test() {
		let source = "test = 1 + 278 >> 2 + 3";
		let tokens = Lexer::new(source).vectorize();
		assert_eq!(
			tokens,
			vec![
				Token {
					kind: TokenKind::Identifier("test".to_string()),
				},
				Token {
					kind: TokenKind::Equals,
				},
				Token {
					kind: TokenKind::NumberLiteral("1".to_string()),
				},
				Token {
					kind: TokenKind::Plus,
				},
				Token {
					kind: TokenKind::NumberLiteral("278".to_string()),
				},
				Token {
					kind: TokenKind::Pipe,
				},
				Token {
					kind: TokenKind::NumberLiteral("2".to_string()),
				},
				Token {
					kind: TokenKind::Plus,
				},
				Token {
					kind: TokenKind::NumberLiteral("3".to_string()),
				},
				Token {
					kind: TokenKind::End,
				}
			]
		);
	}
}
