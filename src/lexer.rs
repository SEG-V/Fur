use std::iter;

pub struct Lexer<'a> {
	column: usize,
	index: usize,
	line: usize,
	text: &'a str
}

#[derive(Debug)]
pub enum Operator {
	Add,
	Divide,
	Multiply,
	Subtract
}

#[derive(Debug)]
pub enum Token {
	Error,
	Integer(u32),
	Operator(Operator),
	Terminator
}

pub struct TokenIterator<'a> {
	lexer: &'a mut Lexer<'a>
}

impl<'a> Lexer<'a> {
	fn get_char(&mut self) -> Option<char> {
		let c = self.text.chars().nth(self.index);

		if self.index < self.text.len() {
			self.index += 1;

			if let Some('\n') = c {
				self.column = 1;
				self.line += 1;
			} else {
				self.column += 1;
			}
		}

		c
	}

	pub fn get_token(&mut self) -> Option<Token> {
		let current_char = self.get_char();

		match current_char {
			Some(mut c) => {
				if c.is_whitespace() {
					while c.is_whitespace() {
						match self.get_char() {
							Some(new_c) => c = new_c,
							None => return None
						}
					}
				}

				match c {
					'+' => Some(Token::Operator(Operator::Add)),
					'/' => Some(Token::Operator(Operator::Divide)),
					'*' => Some(Token::Operator(Operator::Multiply)),
					'-' => Some(Token::Operator(Operator::Subtract)),
					';' => Some(Token::Terminator),

					_ => {
						if c.is_ascii_digit() {
							let mut buffer = String::new();

							loop {
								buffer.push(c);

								match self.text.chars().nth(self.index) {
									Some(new_c) => {
										if new_c.is_ascii_digit() {
											c = self.get_char().unwrap();
										} else {
											break;
										}
									},
									None => break
								}
							}

							return Some(Token::Integer(buffer.parse::<u32>().unwrap()));
						}

						eprintln!("SyntaxError: line {}:{} unsupported character '{}'.", self.line, self.column, c);
						Some(Token::Error)
					}
				}
			}
			None => None
		}
	}

	pub fn new(text: &'a str) -> Lexer<'a> {
		Lexer {
			column: 1,
			index: 0,
			line: 1,
			text
		}
	}

	pub fn tokenize(&'a mut self) -> TokenIterator<'a> {
		TokenIterator {
			lexer: self
		}
	}
}

impl<'a> iter::Iterator for TokenIterator<'a> {
	type Item = Token;

	fn next(&mut self) -> Option<Self::Item> {
		self.lexer.get_token()
	}
}