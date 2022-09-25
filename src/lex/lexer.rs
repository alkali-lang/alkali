use itertools::Itertools;
use std::fs::File;
use utf8_chars::BufReadCharsExt;

use super::{token, Token};
use std::io::{self};

pub fn lex_source(source: &str) -> token::TokenReader {
	let mut file = File::open(source)
		.map(|f| io::BufReader::new(f))
		.expect(format!("Could not open file {}", source).as_str());

	let test = &mut file.chars().map(|c| c.unwrap());

	let tokens = lex(test);
	token::TokenReader::new(tokens)
}

fn lex(iterator: &mut dyn Iterator<Item = char>) -> Vec<Token> {
	let mut iterator = iterator.peekable();
	let mut tokens: Vec<Token> = Vec::new();
	while let Some(char) = iterator.next() {
		match char {
			'=' => tokens.push(Token::Equals),
			'+' => tokens.push(Token::Plus),
			'-' => tokens.push(Token::Minus),
			'*' => tokens.push(Token::Star),
			'/' => tokens.push(Token::Slash),
			'%' => tokens.push(Token::Percent),
			'^' => tokens.push(Token::Caret),
			'&' => tokens.push(Token::Ampersand),
			';' => tokens.push(Token::Semicolon),
			'{' => tokens.push(Token::LeftCurlyBrace),
			'}' => tokens.push(Token::RightCurlyBrace),
			'(' => tokens.push(Token::LeftParenthesis),
			')' => tokens.push(Token::RightParenthesis),
			'"' => {
				let string = iterator
					.by_ref()
					.take_while(|c| *c != '"')
					.collect::<String>();
				tokens.push(Token::StringLiteral(string));
			}
			pipe if pipe == '>' && iterator.peek() == Some(&'>') => {
				iterator.next();
				tokens.push(Token::Pipe)
			}
			'<' => tokens.push(Token::LessThan),
			'>' => tokens.push(Token::GreaterThan),
			alpha if alpha.is_alphabetic() => {
				let mut identifier = alpha.to_string();
				let res: String =
					Itertools::peeking_take_while(iterator.by_ref(), |c| c.is_alphanumeric())
						.collect();

				identifier.push_str(&res);
				tokens.push(Token::Identifier(identifier));
			}
			num if num.is_numeric() => {
				let mut number = num.to_string();
				let res: String =
					Itertools::peeking_take_while(iterator.by_ref(), |c| c.is_numeric()).collect();
				number.push_str(&res);
				tokens.push(Token::NumberLiteral(number));
			}
			_ => (),
		}
	}

	tokens.push(Token::End);
	tokens
}

#[cfg(test)]
mod tests {
	use super::*;

	fn basic() -> Vec<Token> {
		vec![
			Token::Identifier("test".to_string()),
			Token::Equals,
			Token::NumberLiteral("1".to_string()),
			Token::Plus,
			Token::NumberLiteral("2".to_string()),
			Token::End,
		]
	}

	#[test]
	fn test_basic() {
		let source = "test = 1 + 2";
		let tokens = lex(&mut source.chars());
		assert_eq!(tokens, basic());
	}

	#[test]
	fn basic_no_space() {
		let source = "test=1+2";
		let tokens = lex(&mut source.chars());
		assert_eq!(tokens, basic());
	}

	#[test]
	fn literals() {
		let source = "test = \"hello\" + 2";
		let tokens = lex(&mut source.chars());
		assert_eq!(
			tokens,
			vec![
				Token::Identifier("test".to_string()),
				Token::Equals,
				Token::StringLiteral("hello".to_string()),
				Token::Plus,
				Token::NumberLiteral("2".to_string()),
				Token::End,
			]
		);
	}

	#[test]
	fn pipe() {
		let source = "test = 1 >> 2";
		let tokens = lex(&mut source.chars());
		assert_eq!(
			tokens,
			vec![
				Token::Identifier("test".to_string()),
				Token::Equals,
				Token::NumberLiteral("1".to_string()),
				Token::Pipe,
				Token::NumberLiteral("2".to_string()),
				Token::End,
			]
		);
	}
}
