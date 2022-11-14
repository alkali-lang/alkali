use std::error::Error;

mod ast;
mod lex;
mod parse;

fn main() -> Result<(), Box<dyn Error>> {
	let ast = parse::parse("input.txt".to_string(), None)?;

	println!("{:#?}", ast);
	Ok(())
}
