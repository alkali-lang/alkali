use std::{fs::File, borrow::BorrowMut};

use super::{token, Token};
use std::io::{self, BufRead};

pub fn lex(source: &str) -> token::TokenReader {
    let mut tokens: Vec<Token> = Vec::new();

    let file = File::open(source)
      .map(|f| io::BufReader::new(f))
      .expect(format!("Could not open file {}", source).as_str());


    

    for line in file.lines() {
      let line = line.unwrap();
      let mut iterator = line.chars();
      while let Some(char) = iterator.next() {
        match char {
          '=' => tokens.push(Token::Equals),
          '+' => tokens.push(Token::Plus),
          '-' => tokens.push(Token::Minus),
          '*' => tokens.push(Token::Star),
          '/' => tokens.push(Token::Slash),
          '%' => tokens.push(Token::Percent),
          '^' => tokens.push(Token::Caret),
          '&' => tokens.push(Token::Ampersand),
          ';' => tokens.push(Token::Semicolon),
          '<' => tokens.push(Token::LessThan),
          '>' => tokens.push(Token::GreaterThan),
          alpha if alpha.is_alphanumeric() => {
            let mut identifier = alpha.to_string();
            let ch = alpha;

            let res = iterator.take_while(|c| c.is_alphanumeric()).collect();
            identifier.push_str(&res);
          },
          _ => tokens.push(Token::Invalid),
        }
      }
    }
    tokens.push(Token::End);
  token::TokenReader::new(tokens)
  }

  
