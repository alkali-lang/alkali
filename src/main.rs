mod ast;
mod lex;
mod parse;

fn main() {
	parse::parse("input.txt".to_string());
}
