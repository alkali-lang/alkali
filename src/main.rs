mod lex;

use lex::lex_source;

fn main() {
	let token_reader = lex_source("input.txt");

	println!("{:?}", token_reader.tokens);
}
