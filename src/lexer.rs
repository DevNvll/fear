use std::iter::Peekable;
use std::str::Chars;

use super::tokens::{lookup_ident, TokenType};
use super::utils;

pub struct Lexer<'a> {
  input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
  pub fn new(input: &str) -> Lexer {
    Lexer {
      input: input.chars().peekable(),
    }
  }

  fn peek_char_eq(&mut self, ch: char) -> bool {
    match self.input.peek() {
      Some(&peek_ch) => peek_ch == ch,
      None => false,
    }
  }

  fn next_is_letter(&mut self) -> bool {
    match self.input.peek() {
      Some(&ch) => utils::is_letter(ch),
      None => false,
    }
  }

  fn read_identifier(&mut self, first: char) -> String {
    let mut ident = String::new();
    ident.push(first);
    while self.next_is_letter() {
      ident.push(self.input.next().unwrap());
    }
    ident
  }

  fn read_number(&mut self, first: char) -> String {
    let mut number = String::new();
    number.push(first);
    while let Some(&c) = self.input.peek() {
      if !c.is_numeric() {
        break;
      }
      number.push(self.input.next().unwrap());
    }
    number
  }

  pub fn next_token(&mut self) -> TokenType {
    match self.input.next() {
      Some('=') => {
        if self.peek_char_eq('=') {
          self.input.next();
          TokenType::Equal
        } else {
          TokenType::Assign
        }
      }
      Some('!') => {
        if self.peek_char_eq('=') {
          self.input.next();
          TokenType::NotEqual
        } else {
          TokenType::Bang
        }
      }
      Some('+') => TokenType::Plus,
      Some('-') => TokenType::Minus,
      Some('/') => TokenType::Slash,
      Some('*') => TokenType::Asterisk,
      Some('<') => TokenType::LowerThan,
      Some('>') => TokenType::GreaterThan,
      Some(';') => TokenType::Semicolon,
      Some(',') => TokenType::Comma,
      Some('{') => TokenType::LeftBrace,
      Some('}') => TokenType::RightBrace,
      Some('(') => TokenType::LeftParenthesis,
      Some(')') => TokenType::RightParenthesis,
      Some('\n') => TokenType::Linebreak,
      Some(character @ _) => {
        if utils::is_letter(character) {
          let literal = self.read_identifier(character);
          lookup_ident(&literal)
        } else if character.is_numeric() {
          TokenType::Integer(self.read_number(character))
        } else if character == '\"' {
          let mut string = String::new();
          string.push(character);
          loop {
            let next = self.input.next().unwrap();
            string.push(next);
            if next == '\"' {
              break;
            }
          }
          TokenType::StringLiteral(string)
        } else if character.is_whitespace() {
          TokenType::Whitespace
        } else {
          TokenType::Illegal
        }
      }
      None => TokenType::EOF,
    }
  }
}
