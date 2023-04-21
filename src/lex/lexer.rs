use super::{BufCharacterReader, Token, TokenKind};

pub struct Lexer<'a> {
	reader: BufCharacterReader<'a>,
	col: usize,
	row: usize,
	next_token: Option<Token>,
}

/// A lexer for the language
/// Currently this only allows looking ahead one token
impl<'a> Lexer<'a> {
	/// Creates a new `Lexer` from given an object implementing [`std::io::Read`]
	pub fn new(src: &'a mut dyn std::io::Read) -> Lexer {
		let reader = BufCharacterReader::new(src, 10);
		Lexer {
			reader,
			col: 0,
			row: 1,
			next_token: None,
		}
	}

	/// Gets the next token
	fn next_token(&mut self) -> Option<Token> {
		if let Some(token) = self.next_token.take() {
			return Some(token);
		}

		loop {
			let Some(ch) = self.reader.peek() else {
				return self.return_end_of_input_token()
			};

			let token = match ch {
				'=' => self.lex_symbol(TokenKind::Equals),
				'+' => self.lex_symbol(TokenKind::Plus),
				'-' => self.lex_symbol(TokenKind::Minus),
				'*' => self.lex_symbol(TokenKind::Star),
				'/' => self.lex_symbol(TokenKind::Slash),
				'%' => self.lex_symbol(TokenKind::Percent),
				'^' => self.lex_symbol(TokenKind::Caret),
				'&' => self.lex_symbol(TokenKind::Ampersand),
				';' => self.lex_symbol(TokenKind::Semicolon),
				'{' => self.lex_symbol(TokenKind::LBrace),
				'}' => self.lex_symbol(TokenKind::RBrace),
				'(' => self.lex_symbol(TokenKind::LParen),
				')' => self.lex_symbol(TokenKind::RParen),
				'<' => self.lex_symbol(TokenKind::LessThan),
				'>' => self.lex_greater_than_or_pipe(),
				'"' => self.lex_str(),
				ch if ch.is_alphabetic() => self.lex_id(),
				ch if ch.is_numeric() => self.lex_number(),
				'\t' | ' ' | '\n' => {
					self.advance();
					continue;
				}
				other => {
					panic!("Don't know how to parse '{other}'.")
				}
			};

			return Some(token);
		}
	}

	/// Yields the end of input based on the lexer state
	fn return_end_of_input_token(&mut self) -> Option<Token> {
		if self.next_token.is_none() {
			Some(Token {
				kind: TokenKind::End,
				row: self.row,
				col: self.col,
			})
		} else {
			self.next_token.take()
		}
	}

	fn lex_greater_than_or_pipe(&mut self) -> Token {
		self.advance();
		if self.reader.peek() == Some(&'>') {
			let start_col = self.col;
			self.advance();
			Token {
				kind: TokenKind::Pipe,
				col: start_col,
				row: self.row,
			}
		} else {
			Token {
				kind: TokenKind::GreaterThan,
				row: self.row,
				col: self.col,
			}
		}
	}

	fn lex_symbol(&mut self, kind: TokenKind) -> Token {
		self.advance();
		Token {
			kind,
			row: self.row,
			col: self.col,
		}
	}

	fn lex_id(&mut self) -> Token {
		let mut ident = String::from(self.advance().unwrap());
		let start_col = self.col;

		while let Some(ch) = self.reader.peek() {
			if ch.is_alphanumeric() {
				ident.push(*ch);
				self.advance();
			} else {
				break;
			}
		}

		Token {
			kind: TokenKind::Identifier(ident),
			row: self.row,
			col: start_col,
		}
	}

	fn lex_number(&mut self) -> Token {
		let mut number = String::from(self.advance().unwrap());
		let start_col = self.col;

		while let Some(ch) = self.reader.peek() {
			if ch.is_numeric() {
				number.push(*ch);
				self.advance();
			} else {
				break;
			}
		}

		Token {
			kind: TokenKind::NumberLiteral(number),
			row: self.row,
			col: start_col,
		}
	}

	fn lex_str(&mut self) -> Token {
		// Eat the opening quote
		self.advance();
		let start_col = self.col;
		let mut string = String::new();

		while let Some(ch) = self.reader.peek() {
			// Eat the closing quote
			if ch == &'"' {
				self.advance();
				break;
			}

			string.push(*ch);
			self.advance();
		}

		Token {
			kind: TokenKind::StringLiteral(string),
			row: self.row,
			col: start_col,
		}
	}

	// Returns the next token in the input stream, without consuming it
	pub fn peek_token(&mut self) -> Option<&Token> {
		if self.next_token.is_none() {
			self.next_token = self.next_token();
		}

		self.next_token.as_ref()
	}

	// Moves lexer index up
	fn advance(&mut self) -> Option<char> {
		if self.reader.peek() == Some(&'\n') {
			self.row += 1;
			self.col = 0;
		} else {
			self.col += 1;
		}

		self.reader.next()
	}
}

impl<'a> Iterator for Lexer<'a> {
	type Item = Token;

	fn next(&mut self) -> Option<Token> {
		if let Some(token) = self.peek_token() {
			if token.kind == TokenKind::End {
				return None;
			}
		}

		self.next_token()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn number() {
		let source = &mut "123".as_bytes();
		let lexer = Lexer::new(source);

		assert_eq!(
			lexer.collect::<Vec<_>>(),
			vec![Token {
				kind: TokenKind::NumberLiteral("123".into()),
				row: 1,
				col: 1
			}],
		);
	}

	#[test]
	fn test_basic() {
		let source = &mut "test = 1 + 278".as_bytes();
		let lexer = Lexer::new(source);
		assert_eq!(
			lexer.collect::<Vec<_>>(),
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
		let source = &mut "let x = \"hello world\"".as_bytes();
		let lexer = &mut Lexer::new(source);
		assert_eq!(
			lexer.collect::<Vec<_>>(),
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
					col: 9
				},
			]
		);
	}

	#[test]
	fn test_greater_than() {
		let source = &mut "test = 1 > 278".as_bytes();
		let lexer = Lexer::new(source);
		assert_eq!(
			lexer.collect::<Vec<_>>(),
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
		let source = &mut "test = 1 + 278\n2 + 3".as_bytes();
		let lexer = Lexer::new(source);
		assert_eq!(
			lexer.collect::<Vec<_>>(),
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
		let source = &mut "test = 1 + 278 >> 2 + 3".as_bytes();
		let lexer = Lexer::new(source);
		assert_eq!(
			lexer.collect::<Vec<_>>(),
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
