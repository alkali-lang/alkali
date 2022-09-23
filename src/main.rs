mod lex;

use lex::lex;

fn main() {
    let mut token_reader = lex("input.txt");

    while !token_reader.end_of_file() {
        println!("{:?}", token_reader.next_token());
    }
}
