use std::fs::File;

mod ast;
mod lex;
mod parse;
mod shared;

fn main() -> shared::Result<()> {
	let mut file = File::open("input.txt").unwrap();
	let lexer = lex::Lexer::new(&mut file);

	Ok(())
}
