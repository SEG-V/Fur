use std::{io, io::Write};
use std::process;

mod lexer;

fn main() {
	start_repl();
}

fn start_repl() {
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

		let mut lexer = lexer::Lexer::new(input.trim());

		for token in lexer.tokenize() {
			println!("{:?}", token)
		}
	}
}