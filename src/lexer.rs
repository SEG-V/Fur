use crate::token;

#[derive(Copy, Clone)]
pub struct Lexer<'text> {
	position: crate::Position,
	text: &'text str
}

impl<'text> Lexer<'text> {
	pub fn drop_token(&mut self) {
		self.get_token();
	}

	fn get_char(&mut self) -> (Option<char>, crate::Position) {
		let position = self.position;
		let c = self.text.chars().nth(self.position.index);

		if self.position.index < self.text.len() {
			self.position.index += 1;

			if let Some('\n') = c {
				self.position.column = 1;
				self.position.line += 1;
			} else {
				self.position.column += 1;
			}
		}

		
		(c, position)
	}

	pub fn get_token(&mut self) -> crate::token::Token {
		let (current_char, mut position) = self.get_char();

		match current_char {
			Some(mut c) => {
				if c.is_whitespace() {
					while c.is_whitespace() {
						match self.get_char() {
							(Some(new_c), new_position) => {
								c = new_c;
								position = new_position;
							},
							(None, _) => return crate::token::Token::EOF(position)
						}
					}
				}

				match c {
					'+' => token::Token::Operator(position, crate::Operator::Add),
					'/' => token::Token::Operator(position, crate::Operator::Divide),
					'*' => token::Token::Operator(position, crate::Operator::Multiply),
					'-' => token::Token::Operator(position, crate::Operator::Subtract),
					'(' => token::Token::ParenthesisL(position),
					')' => token::Token::ParenthesisR(position),
					';' => token::Token::Terminator(position),

					_ => {
						if c.is_ascii_digit() {
							let mut buffer = String::new();

							loop {
								buffer.push(c);

								match self.text.chars().nth(self.position.index) {
									Some(new_c) => {
										if new_c.is_ascii_digit() {
											let wrapped_c_and_position = self.get_char();
											c = wrapped_c_and_position.0.unwrap();
											position = wrapped_c_and_position.1;
										} else {
											break;
										}
									},
									None => break
								}
							}

							return crate::token::Token::Integer(position, buffer.parse::<u64>().unwrap());
						}

						eprintln!("SyntaxError: line {}:{}, unsupported character '{}'.", position.line, position.column, c);
						crate::token::Token::Error(position)
					}
				}
			}
			None => crate::token::Token::EOF(position)
		}
	}

	pub fn new(text: &'text str) -> Self {
		Self {
			position: crate::Position {
				column: 1,
				index: 0,
				line: 1
			},
			text
		}
	}

	pub fn view_token(&self) -> token::Token {
		let mut lexer = *self;
		lexer.get_token()
	}
}