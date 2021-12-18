pub mod ast;
pub mod lexer;
pub mod parser;
pub mod token;

use std::fmt;
use std::{io, io::Write};
use std::process;

pub enum Operator {
	Add,
	Divide,
	Multiply,
	Subtract
}

#[derive(Copy, Clone)]
pub struct Position {
	pub column: usize,
	pub index: usize,
	pub line: usize
}

impl Operator {
	pub fn is_binary(&self) -> bool {
		match self {
			Operator::Add => true,
			Operator::Divide => true,
			Operator::Multiply => true,
			Operator::Subtract => true
		}
	}

	pub fn is_unary(&self) -> bool {
		match self {
			Operator::Subtract => true,
			_ => false
		}
	}
}

impl fmt::Display for Operator {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Operator::Add => write!(f, "'+'"),
			Operator::Divide => write!(f, "'/'"),
			Operator::Multiply => write!(f, "'*'"),
			Operator::Subtract => write!(f, "'-'")
		}
	}
}

pub fn start_repl() {
	println!("SPARTAN (version {})", env!("CARGO_PKG_VERSION"));

	loop {
		let mut input = String::new();

		print!("> ");
		match io::stdout().flush() {
			Ok(_) => (), // do nothing
			Err(_) => {
				eprintln!("ERROR: failed to write to stdout.");
				process::exit(1);
			}
		}

		match io::stdin().read_line(&mut input) {
			Ok(n) => {
				if n == 0 {
					continue;
				} else if input.trim() == "exit" {
					break;
				}
			},
			Err(_) => {
				eprintln!("ERROR: failed to read from stdin.");
				process::exit(1);
			}
		}

		parser::Parser::new(input.trim()).parse();
	}
}