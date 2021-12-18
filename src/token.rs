use std::fmt;

pub enum Token {
	Error(crate::Position),
	EOF(crate::Position),
	Integer(crate::Position, u64),
	Operator(crate::Position, crate::Operator),
	ParenthesisL(crate::Position),
	ParenthesisR(crate::Position),
	Terminator(crate::Position)
}

impl Token {
	pub fn position(&self) -> &crate::Position {
		match self {
			Token::Error(position) => position,
			Token::EOF(position) => position,
			Token::Integer(position, _) => position,
			Token::Operator(position, _) => position,
			Token::ParenthesisL(position) => position,
			Token::ParenthesisR(position) => position,
			Token::Terminator(position) => position
		}
	}
}

impl fmt::Display for Token {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Token::Error(_) => write!(f, "Token::Error"),
			Token::EOF(_) => write!(f, "EOF (End-Of-File)"),
			Token::Integer(_, _) => write!(f, "integer"),
			Token::Operator(_, operator) => write!(f, "{}", operator),
			Token::ParenthesisL(_) => write!(f, "'('"),
			Token::ParenthesisR(_) => write!(f, "')'"),
			Token::Terminator(_) => write!(f, "';'")
		}
	}
}