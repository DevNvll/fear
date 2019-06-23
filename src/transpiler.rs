use super::tokens::TokenType;

pub fn transpile(tokens: &Vec<TokenType>) -> String {
  let mut code = String::new();
  let mut peekable_tokens = tokens.iter().peekable();

  loop {
    let cur = peekable_tokens.next().unwrap();
    match cur {
      TokenType::Ident(i) => {
        code.push_str(i.as_str());
      }
      TokenType::Integer(i) => {
        code.push_str(i.as_str());
      }
      TokenType::StringLiteral(s) => {
        code.push_str(s);
      }
      TokenType::Global(global) => match global.as_ref() {
        "print" => code.push_str("console.log"),
        _ => (),
      },
      TokenType::Whitespace => {
        code.push_str(" ");
      }
      TokenType::Linebreak => {
        code.push_str("\n");
      }
      TokenType::Let => {
        code.push_str("let");
      }
      TokenType::Plus => {
        code.push_str("+");
      }
      TokenType::Minus => {
        code.push_str("-");
      }
      TokenType::Bang => {
        code.push_str("!");
      }
      TokenType::Asterisk => {
        code.push_str("*");
      }
      TokenType::Return => {
        code.push_str("return");
      }
      TokenType::Comma => {
        code.push_str(",");
      }
      TokenType::Slash => {
        code.push_str("/");
      }
      TokenType::Assign => {
        code.push_str("=");
      }
      TokenType::Semicolon => {
        code.push_str(";");
      }
      TokenType::Function => {
        code.push_str("function");
      }
      TokenType::LeftParenthesis => {
        code.push_str("(");
      }
      TokenType::RightParenthesis => {
        code.push_str(")");
      }
      TokenType::LeftBrace => {
        code.push_str("{");
      }
      TokenType::RightBrace => {
        code.push_str("}");
      }
      TokenType::LowerThan => {
        code.push_str("<");
      }
      TokenType::GreaterThan => {
        code.push_str(">");
      }
      TokenType::NotEqual => {
        code.push_str("!=");
      }
      TokenType::If => {
        code.push_str("if");
      }
      TokenType::Else => {
        code.push_str("else");
      }
      TokenType::True => {
        code.push_str("true");
      }
      TokenType::False => {
        code.push_str("false");
      }
      TokenType::EOF => break,
      _ => break,
    }
  }
  code
}
