use std::{error::Error, fs::read_to_string};

use crate::{
	ast::{BinaryOp, Expr, ExprKind, LetDecl, Stmt, StmtKind},
	lex::{Lexer, TokenKind},
};

#[derive(Debug, PartialEq)]
pub struct SourceFile {
	pub stmts: Vec<Stmt>,
}

impl SourceFile {
	fn new() -> Self {
		Self { stmts: Vec::new() }
	}
}

pub fn parse(src: String, file: Option<bool>) -> Result<SourceFile, Box<dyn Error>> {
	let mut root = SourceFile::new();

	let str = if file.unwrap_or(true) {
		read_to_string(src).unwrap()
	} else {
		src
	};

	let mut lexer = Lexer::new(str.as_str());

	while lexer.peek_token().kind != TokenKind::End {
		let token = lexer.peek_token();

		match token.kind {
			TokenKind::Identifier(ident) => {
				if ident == "let" {
					lexer.next()?;
					let let_decl = parse_let(&mut lexer)?;

					root.stmts.push(let_decl);
				}
			}
			_ => todo!("Parse error {:?}", token.kind),
		};

		if lexer.peek_token().kind == TokenKind::End {
			break;
		}

		// lexer.next();
	}

	Ok(root)
}

pub fn parse_term(lexer: &mut Lexer) -> Result<Expr, Box<dyn Error>> {
	let mut expr = parse_factor(lexer)?;

	while lexer.peek_token().kind == TokenKind::Plus || lexer.peek_token().kind == TokenKind::Minus
	{
		let token = lexer.peek_token();

		match token.kind {
			TokenKind::Plus => {
				lexer.next()?;
				let right = parse_factor(lexer)?;
				expr = Expr {
					kind: ExprKind::Binary(BinaryOp::Add, Box::new(expr), Box::new(right)),
				};
			}
			TokenKind::Minus => {
				lexer.next()?;
				let right = parse_factor(lexer)?;
				expr = Expr {
					kind: ExprKind::Binary(BinaryOp::Subtract, Box::new(expr), Box::new(right)),
				};
			}
			_ => todo!(),
		}
	}

	Ok(expr)
}

pub fn parse_expr(lexer: &mut Lexer) -> Result<Expr, Box<dyn Error>> {
	parse_term(lexer)
}

pub fn parse_primary(lexer: &mut Lexer) -> Result<Expr, Box<dyn Error>> {
	let token = lexer.peek_token();

	match token.kind {
		TokenKind::NumberLiteral(num) => {
			lexer.next()?;
			Ok(Expr {
				kind: ExprKind::NumLit(num.parse().unwrap()),
			})
		}
		TokenKind::StringLiteral(str) => {
			lexer.next()?;
			Ok(Expr {
				kind: ExprKind::StrLit(str),
			})
		}
		TokenKind::Identifier(ident) => {
			lexer.next()?;
			Ok(Expr {
				kind: ExprKind::Reference(ident),
			})
		}
		TokenKind::LParen => {
			// Consume the left paren
			lexer.next()?;

			// Parse the expression within the parens
			let expr = parse_expr(lexer)?;

			// Check for the closing paren
			if lexer.peek_token().kind != TokenKind::RParen {
				return Err("Expected closing paren".into());
			}

			// Consume the right paren
			lexer.next()?;

			Ok(Expr {
				kind: ExprKind::Group(Box::new(expr)),
			})
		}
		_ => Err(format!("Expected unary token, got {:?}", token.kind).into()),
	}
}

pub fn parse_factor(lexer: &mut Lexer) -> Result<Expr, Box<dyn Error>> {
	let mut unary = parse_primary(lexer)?;

	while lexer.peek_token().kind == TokenKind::Slash || lexer.peek_token().kind == TokenKind::Star
	{
		let token = lexer.peek_token();

		match token.kind {
			TokenKind::Slash => {
				lexer.next()?;
				let right = parse_primary(lexer)?;
				unary = Expr {
					kind: ExprKind::Binary(BinaryOp::Divide, Box::new(unary), Box::new(right)),
				};
			}
			TokenKind::Star => {
				lexer.next()?;
				let right = parse_primary(lexer)?;
				unary = Expr {
					kind: ExprKind::Binary(BinaryOp::Multiply, Box::new(unary), Box::new(right)),
				};
			}
			_ => return Err(format!("Expected factor token, got {:?}", token.kind).into()),
		}
	}

	Ok(unary)
}

pub fn parse_let(lexer: &mut Lexer) -> Result<Stmt, Box<dyn Error>> {
	let token = lexer.peek_token();

	if let TokenKind::Identifier(name) = token.kind {
		lexer.next()?;
		let equals = lexer.peek_token();

		match equals.kind {
			TokenKind::Equals => (),
			_ => panic!("Unexpected token: {:?}", equals),
		}

		lexer.next()?;

		let value = Box::new(parse_expr(lexer)?);
		let decl = Box::new(LetDecl { name, value });

		if lexer.peek_token().kind != TokenKind::Semicolon {
			panic!("Expected semicolon,(saw {:?}", lexer.peek_token().kind);
		}

		lexer.next()?;

		Ok(Stmt {
			kind: StmtKind::LetDecl(decl),
		})
	} else {
		panic!("Expected identifier, got {:?}", token);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn basic_decl() {
		let ast = parse("let x = 1;".to_string(), Some(false)).unwrap();

		assert_eq!(
			ast,
			SourceFile {
				stmts: vec![Stmt {
					kind: StmtKind::LetDecl(Box::new(LetDecl {
						name: "x".to_string(),
						value: Box::new(Expr {
							kind: ExprKind::NumLit(1.0)
						})
					}))
				}]
			}
		);
	}

	#[test]
	fn binary_plus() {
		let ast = parse("let x = 1 + 2;".to_string(), Some(false)).unwrap();

		assert_eq!(
			ast,
			SourceFile {
				stmts: vec![Stmt {
					kind: StmtKind::LetDecl(Box::new(LetDecl {
						name: "x".to_string(),
						value: Box::new(Expr {
							kind: ExprKind::Binary(
								BinaryOp::Add,
								Box::new(Expr {
									kind: ExprKind::NumLit(1.0)
								}),
								Box::new(Expr {
									kind: ExprKind::NumLit(2.0)
								})
							)
						})
					}))
				}]
			}
		);
	}

	#[test]
	fn binary_minus() {
		let ast = parse("let x = 1 - 2;".to_string(), Some(false)).unwrap();

		assert_eq!(
			ast,
			SourceFile {
				stmts: vec![Stmt {
					kind: StmtKind::LetDecl(Box::new(LetDecl {
						name: "x".to_string(),
						value: Box::new(Expr {
							kind: ExprKind::Binary(
								BinaryOp::Subtract,
								Box::new(Expr {
									kind: ExprKind::NumLit(1.0)
								}),
								Box::new(Expr {
									kind: ExprKind::NumLit(2.0)
								})
							)
						})
					}))
				}]
			}
		);
	}

	#[test]
	fn multiline() {
		let ast = parse("let x = 1;\nlet y = 2 + 2;".to_string(), Some(false)).unwrap();

		assert_eq!(
			ast,
			SourceFile {
				stmts: vec![
					Stmt {
						kind: StmtKind::LetDecl(Box::new(LetDecl {
							name: "x".to_string(),
							value: Box::new(Expr {
								kind: ExprKind::NumLit(1.0)
							})
						}))
					},
					Stmt {
						kind: StmtKind::LetDecl(Box::new(LetDecl {
							name: "y".to_string(),
							value: Box::new(Expr {
								kind: ExprKind::Binary(
									BinaryOp::Add,
									Box::new(Expr {
										kind: ExprKind::NumLit(2.0)
									}),
									Box::new(Expr {
										kind: ExprKind::NumLit(2.0)
									})
								)
							})
						}))
					}
				]
			}
		);
	}

	#[test]
	pub fn precedence() {
		let ast = parse("let x = 1 + 2 * 3;".to_string(), Some(false)).unwrap();

		assert_eq!(
			ast,
			SourceFile {
				stmts: vec![Stmt {
					kind: StmtKind::LetDecl(Box::new(LetDecl {
						name: "x".to_string(),
						value: Box::new(Expr {
							kind: ExprKind::Binary(
								BinaryOp::Add,
								Box::new(Expr {
									kind: ExprKind::NumLit(1.0)
								}),
								Box::new(Expr {
									kind: ExprKind::Binary(
										BinaryOp::Multiply,
										Box::new(Expr {
											kind: ExprKind::NumLit(2.0)
										}),
										Box::new(Expr {
											kind: ExprKind::NumLit(3.0)
										})
									)
								})
							)
						})
					}))
				}]
			}
		);
	}
}
