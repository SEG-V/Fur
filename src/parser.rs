use std::fmt;

use crate::ast;
use crate::token;
use crate::lexer;

struct Error {
	pub kind: ErrorKind,
	message: String
}

enum ErrorKind {
	Abort,
	SyntaxError
}

pub struct Parser<'lexer> {
	lexer: lexer::Lexer<'lexer>
}

enum PrecedenceLevel {
	LOW,
	MID,
	MAX
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}: {}", match &self.kind {
			ErrorKind::Abort => "Abort",
			ErrorKind::SyntaxError => "SyntaxError"
		}, self.message)
	}
}

impl<'input> Parser<'input> {
	pub fn new(input: &'input str) -> Parser<'input> {
		Parser {
			lexer: crate::lexer::Lexer::new(input)
		}
	}

	pub fn parse(&mut self) -> ast::Program {
		let mut statements: Vec<ast::Statement> = Vec::new();

		loop {
			match self.parse_statement() {
				Ok(statement) => statements.push(statement),
				Err(error) => match error.kind {
					ErrorKind::Abort => {
						break;
					},
					ErrorKind::SyntaxError => eprintln!("{}", error)
				}
			}
		}

		ast::Program { statements }
	}

	fn parse_expr(&mut self, precedence: PrecedenceLevel) -> Result<ast::Expr, Error> {
		match precedence {
			PrecedenceLevel::LOW
			| PrecedenceLevel::MID => {
				let mut expr = self.parse_expr(precedence.next())?;

				loop {
					match self.lexer.view_token() {
						token::Token::Operator(_, operator) if operator.is_binary() => {
							match (&operator, &precedence) {
								(
									crate::Operator::Add
									| crate::Operator::Subtract,
									PrecedenceLevel::LOW
								) => {
									self.lexer.drop_token();
									expr = ast::Expr::BinaryOperation(
										operator,
										Box::new(expr),
										Box::new(
											self.parse_expr(precedence.next())?
										)
									);
								},
								(
									crate::Operator::Divide
									| crate::Operator::Multiply,
									PrecedenceLevel::MID
								) => {
									self.lexer.drop_token();
									expr = ast::Expr::BinaryOperation(
										operator,
										Box::new(expr),
										Box::new(
											self.parse_expr(precedence.next())?
										)
									);
								}
								_ => break
							}
						},
						token::Token::Operator(position, operator) => return Err(Error {
							kind: ErrorKind::SyntaxError,
							message: format!("Line {}:{}, expected a binary operator, got {}.",
								position.line,
								position.column,
								operator
							)
						}),
						_ => break
					}
				}
				Ok(expr)
			},
			PrecedenceLevel::MAX => Ok(ast::Expr::Value(
				self.parse_value()?
			))
		}
	}

	fn parse_statement(&mut self) -> Result<ast::Statement, Error> {
		match self.lexer.view_token() {
			token::Token::EOF(_) => return Err(Error {
				kind: ErrorKind::Abort,
				message: "Reached EOF (End-Of-File).".to_owned()
			}),
			_ => ()
		}

		let expr = self.parse_expr(PrecedenceLevel::LOW)?;

		match self.lexer.get_token() {
			token::Token::Error(_) => return Err(Error {
				kind: ErrorKind::Abort,
				message: "Found an Error Token.".to_owned()
			}),
			token::Token::Terminator(_) => (),
			token => return Err(Error {
				kind: ErrorKind::SyntaxError,
				message: format!("Line {}:{}, expected ';', got {}.",
					token.position().line,
					token.position().column,
					token
				)
			})
		}

		Ok(ast::Statement {
			expr
		})
	}

	fn parse_value(&mut self) -> Result<ast::Value, Error> {
		match self.lexer.get_token() {
			token::Token::Error(_) => Err(Error {
				kind: ErrorKind::Abort,
				message: "Found an Error Token.".to_owned()
			}),
			token::Token::Integer(_, value) => Ok(ast::Value::Integer(value)),
			token::Token::Operator(_, operator) if operator.is_unary() => Ok(ast::Value::UnaryOperation(
				operator, Box::new(self.parse_value()?)
			)),
			token::Token::ParenthesisL(_) => {
				if let token::Token::ParenthesisR(_) = self.lexer.view_token() {
					self.lexer.drop_token();
					return Ok(ast::Value::Null)
				}

				let value = ast::Value::Expr(
					Box::new(self.parse_expr(PrecedenceLevel::LOW)?)
				);

				match self.lexer.get_token() {
					token::Token::Error(_) => return Err(Error {
						kind: ErrorKind::Abort,
						message: "Found an Error Token.".to_owned()
					}),
					token::Token::ParenthesisR(_) => (),
					token => return Err(Error {
						kind: ErrorKind::SyntaxError,
						message: format!("Line {}:{}, expected ')', got {}.",
							token.position().line,
							token.position().column,
							token
						)
					})
				};
				Ok(value)
			}
			token => Err(Error {
				kind: ErrorKind::SyntaxError,
				message: format!("Line {}:{}, expected a Value, got {}.",
					token.position().line,
					token.position().column,
					token
				)
			})
		}
	}
}

impl PrecedenceLevel {
	pub fn next(&self) -> Self {
		match self {
			Self::LOW => Self::MID,
			Self::MID
			| Self::MAX => Self::MAX
		}
	}
}