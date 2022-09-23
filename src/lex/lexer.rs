use std::fs::File;

use super::{token, Token};
use std::io::{self, BufRead};

pub fn lex(source: &str) -> token::TokenReader {
	let mut tokens: Vec<Token> = Vec::new();

	let file = File::open(source)
		.map(|f| io::BufReader::new(f))
		.expect(format!("Could not open file {}", source).as_str());

	for line in file.lines() {
		let line = line.unwrap();
		for ch in line.chars() {
			match ch {
				'=' => tokens.push(Token::Equals),
				'+' => tokens.push(Token::Plus),
				'-' => tokens.push(Token::Minus),
				'*' => tokens.push(Token::Star),
				'/' => tokens.push(Token::Slash),
				'%' => tokens.push(Token::Percent),
				'^' => tokens.push(Token::Caret),
				'&' => tokens.push(Token::Ampersand),
				' ' => tokens.push(Token::Space),
				'\t' => tokens.push(Token::Tab),
				'\n' => tokens.push(Token::Newline),
				';' => tokens.push(Token::Semicolon),
				'<' => tokens.push(Token::LessThan),
				'>' => tokens.push(Token::GreaterThan),
				alpha if alpha.is_alphanumeric() => tokens.push(Token::Alpha(alpha)),
				_ => tokens.push(Token::String(ch.to_string())),
			}
		}
	}

	tokens.push(Token::End);
	token::TokenReader::new(tokens)
}
