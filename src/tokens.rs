#[derive(Debug, PartialEq)]
pub enum TokenType {
  // Literals are stored as strings
  Ident(String),
  Global(String),
  Integer(String),
  StringLiteral(String),

  // Operators
  Assign,
  Plus,
  Minus,
  Bang,
  Asterisk,
  Slash,
  LowerThan,
  GreaterThan,
  Equal,
  NotEqual,

  Comma,
  Semicolon,
  LeftParenthesis,
  RightParenthesis,
  LeftBrace,
  RightBrace,
  Whitespace,
  Linebreak,

  // Keywords
  Function,
  Let,
  True,
  False,
  If,
  Else,
  Return,
  Illegal,
  EOF,
}

impl Default for TokenType {
  fn default() -> TokenType {
    TokenType::Illegal
  }
}

fn is_global(ident: &str) -> bool {
  match ident {
    "print" => true,
    _ => false,
  }
}

pub fn lookup_ident(ident: &str) -> TokenType {
  match ident {
    "fn" => TokenType::Function,
    "seja" | "let" => TokenType::Let,
    "verdadeiro" | "true" => TokenType::True,
    "falso" | "false" => TokenType::False,
    "se" | "if" => TokenType::If,
    "senao" | "else" => TokenType::Else,
    "retorna" | "return" => TokenType::Return,
    _ => {
      if is_global(ident) {
        TokenType::Global(ident.to_string())
      } else {
        TokenType::Ident(ident.to_string())
      }
    }
  }
}
