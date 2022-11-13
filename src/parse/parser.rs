use std::fs::read_to_string;

use crate::{
	ast::{Expr, ExprKind, LetDecl, Stmt, StmtKind},
	lex::{Lexer, TokenKind, TokenReader},
};

pub fn parse(src: String) {
	let file_str = read_to_string(src).expect("test");
	let mut reader = TokenReader::new(Lexer::new(file_str.as_str()).vectorize());

	let mut token = reader.peek();

	while token.kind != TokenKind::End {
		match token.kind {
			TokenKind::Identifier(_) => parse_ident(&mut reader),
			_ => panic!("Unexpected token: {:?}", token),
		};

		if reader.peek().kind == TokenKind::End {
			break;
		}

		token = reader.next_token();
	}
}

pub fn parse_ident(reader: &mut TokenReader) -> Box<Stmt> {
	let token = reader.peek();

	match &token.kind {
		TokenKind::Identifier(ident) => {
			if ident == "let" {
				parse_let(reader)
			} else {
				panic!("Unexpected token: {:?}", token);
			}
		}
		_ => panic!("Unexpected token: {:?}", token),
	}
}

pub fn parse_expr(reader: &mut TokenReader) -> Box<Expr> {
	let token = reader.next_token();

	match &token.kind {
		TokenKind::NumberLiteral(num_str) => Box::new(Expr {
			kind: ExprKind::NumLit(num_str.parse::<f64>().unwrap()),
		}),
		TokenKind::StringLiteral(str) => Box::new(Expr {
			kind: ExprKind::StrLit(str.clone()),
		}),
		_ => panic!("Unexpected token: {:?}", token),
	}
}

pub fn parse_let(reader: &mut TokenReader) -> Box<Stmt> {
	let token = reader.next_token();

	if let TokenKind::Identifier(name) = token.kind {
		let equals = reader.next_token();

		match equals.kind {
			TokenKind::Equals => (),
			_ => panic!("Unexpected token: {:?}", equals),
		}

		let value = parse_expr(reader);
		let decl = Box::new(LetDecl { name, value });
		let stmt = Box::new(Stmt {
			kind: StmtKind::LetDecl(decl),
		});

		println!("{:#?}", stmt);

		stmt
	} else {
		panic!("Expected identifier, got {:?}", token);
	}
}
