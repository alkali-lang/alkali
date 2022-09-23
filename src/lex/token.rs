use std::fmt::Debug;


#[derive(Debug, PartialEq)]
pub enum Token {
  Alpha(char),
  Equals,
  Plus,
  Minus,
  Star,
  Slash,
  Percent,
  Caret,
  Ampersand,
  Space,
  Newline,
  String(String),
  Tab,
  End,
  Semicolon,
  LessThan,
  GreaterThan,
}

pub struct TokenReader {
  pub tokens: Vec<Token>,
  pub position: usize,
}

impl TokenReader {
  pub fn new(tokens: Vec<Token>) -> TokenReader {
    TokenReader {
      tokens,
      position: 0,
    }
  }

  pub fn peek(&self) -> &Token {
    self.tokens.get(self.position).unwrap()
  }

  pub fn next_tokens(&mut self, n: usize) -> &Token {
    self.position += n;
    self.next_token()
  }

  pub fn end_of_file(&self) -> bool {
    self.peek() == &Token::End
  }

  pub fn next_token(&mut self) -> &Token {
    self.position += 1;
    let token = self.peek();
    token
  }
}