mod lexer;
mod tokens;
mod transpiler;
mod utils;

use std::env;
use std::process::Command;

use transpiler::transpile;

fn run(code: String) {
  Command::new("node")
    .arg("-e")
    .arg(code)
    .status()
    .expect("Erro ao executar o script.");
}

fn main() {
  let args: Vec<String> = env::args().collect();
  match args.len() {
    1 => println!("You must provide a file"),
    2 => {
      let mut file: String = utils::read_file(&args[1]);
      let tokens: Vec<tokens::TokenType> = utils::parse_tokens_from_file(&mut file);
      let code: String = transpile(&tokens);
      run(code);
    }
    3 => {
      let mut file: String = utils::read_file(&args[1]);
      let tokens: Vec<tokens::TokenType> = utils::parse_tokens_from_file(&mut file);
      let code: String = transpile(&tokens);
      let output_path: String = format!("{}", args[2]);
      utils::write_to_file(output_path, code);
    }
    _ => (),
  }
}
