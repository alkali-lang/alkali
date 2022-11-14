mod ast;
mod lex;
mod parse;

fn main() {
	let ast = parse::parse("input.txt".to_string(), None);
	println!("{:#?}", ast);
}
