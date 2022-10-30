use crate::{
	ast::{Decl, Expr},
	lex::{Lexer, TokenKind, TokenReader},
};

pub fn is_reserved(str: &str) -> bool {
	return vec!["let"].contains(&str);
}

pub fn parse(src: String) {
	let mut reader = TokenReader::new(Lexer::new(src.as_str()).vectorize());

	while reader.peek().kind != TokenKind::End {
		println!("{:?}", decl);
	}
}
