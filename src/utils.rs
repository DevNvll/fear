use super::lexer::Lexer;
use super::tokens::TokenType;
use std::fs;

pub fn write_to_file(path: String, code: String) {
  fs::write(path, code).expect("Erro ao escrever arquivo de saída.");
}

pub fn read_file(file: &str) -> String {
  fs::read_to_string(file).unwrap()
}

pub fn parse_tokens_from_file(file_contents: &mut String) -> Vec<TokenType> {
  let mut lexer = Lexer::new(&file_contents);
  let mut token_list = Vec::new();
  loop {
    let token: TokenType = lexer.next_token();

    if token == TokenType::EOF {
      token_list.push(token);
      break;
    }
    token_list.push(token);
  }
  token_list
}

pub fn is_letter(ch: char) -> bool {
  ch.is_alphabetic() || ch == '_'
}
