mod ast;
mod lexer;
mod parser;
mod runtime;
mod token;

use std::fmt;
use std::{ io, io::Write };
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
		matches!(self, Operator::Subtract)
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
	println!("{} (version {})",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
	let mut rt = runtime::Runtime::new();
	
	loop {
		let mut input = String::new();

		print!("> ");
		match io::stdout().flush() {
			Ok(_) => (),
			Err(_) => {
				eprintln!("ERROR: failed to write to stdout.");
				process::exit(1);
			}
		}
		
		match io::stdin().read_line(&mut input) {
			Ok(_) => {
				if input.trim() == "exit" {
					break;
				}
			},
			Err(_) => {
				eprintln!("ERROR: failed to read from stdin.");
				process::exit(1);
			}
		}
		
		rt.run(
			"<core>",
			&parser::Parser::new(input.trim()).parse()
		);
	}
}
