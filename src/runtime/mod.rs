use std::ops::Deref;
use std::process;

use crate::ast;

mod utils;
use utils::{Data, DataKind, DataValue, Error, ErrorKind, Frame, Integer, Null};

macro_rules! operate {
	($operator:expr, $x:expr, $y:expr) => {
		match $operator {
			crate::Operator::Add => utils::add($x, $y),
			crate::Operator::Divide => {
				if $y == 0 {
					return Err(Error {
						kind: ErrorKind::OperationError,
						message: "Cannot divide by zero.".to_owned()
					})
				}
				utils::div($x, $y)
			},
			crate::Operator::Multiply => utils::mul($x, $y),
			crate::Operator::Subtract => utils::sub($x, $y)
		}
	};
}

pub struct Runtime {
	counter: usize,
	stack: Vec<Frame>
}

impl Runtime {
	pub fn new() -> Self {
		Self {
			counter: 0,
			stack: Vec::new()
		}
	}

	fn evalute_expr(&self, expr: &ast::Expr) -> Result<Box<dyn Data>, Error> {
		match expr {
			ast::Expr::BinaryOperation(operator, lhs, rhs) => {
				let x = self.evalute_expr(lhs)?;
				let y = self.evalute_expr(rhs)?;

				match (x.kind(), y.kind()) {
					(DataKind::Integer, DataKind::Integer) => {
						match (x.value(), y.value()) {
							(DataValue::I64(xv), DataValue::I64(yv)) => return Ok(Box::new(
								Integer {
									value: operate!(operator, xv, yv)
								}
							)),
							(DataValue::I64(xv), DataValue::U64(yv)) => return Ok(Box::new(
								Integer {
									value: operate!(operator, xv, yv as i64)
								}
							)),
							(DataValue::U64(xv), DataValue::U64(yv)) => return Ok(Box::new(
								Integer {
									value: operate!(operator, xv, yv)
								}
							)),
							(DataValue::U64(xv), DataValue::I64(yv)) => return Ok(Box::new(
								Integer {
									value: operate!(operator, xv as i64, yv)
								}
							)),
							_ => return Err(Error {
								kind: ErrorKind::InternalError,
								message: "Unexpected behaviour at Run#Expr#BinaryOperation.".to_owned()
							})
						}
					},
					(DataKind::Null, kind) => return Err(Error {
						kind: ErrorKind::OperationError,
						message: format!("{} operator is not supported for types: Null and {}.",
							operator,
							kind
						)
					}),
					(kind, DataKind::Null) => return Err(Error {
						kind: ErrorKind::OperationError,
						message: format!("{} operator is not supported for types: {} and Null.",
							operator,
							kind
						)
					})
				}
			},
			ast::Expr::Value(value) => self.evalute_value(value)
		}
	}

	fn evalute_statement(&self, statement: &ast::Statement) -> Result<Box<dyn Data>, Error> {
		self.evalute_expr(&statement.expr)
	}

	fn evalute_value(&self, value: &ast::Value) -> Result<Box<dyn Data>, Error> {
		match value {
			ast::Value::Expr(expr) => self.evalute_expr(expr.deref()),
			ast::Value::Integer(value) => Ok(Box::new(
				Integer {
					value: *value
				}
			)),
			ast::Value::Null => Ok(Box::new(Null {})),
			ast::Value::UnaryOperation(operator, rhs) => {
				match operator {
					crate::Operator::Subtract => {
						let operand = self.evalute_value(rhs)?;

						match operand.kind() {
							DataKind::Integer => {
								match operand.value() {
									DataValue::I64(value) => return Ok(Box::new(
										Integer {
											value: -value
										}
									)),
									DataValue::U64(value) => return Ok(Box::new(
										Integer {
											value: -(value as i64)
										}
									)),
									_ => Err(Error {
										kind: ErrorKind::InternalError,
										message: "Unexpected behaviour at Run#Value#UnaryOperation.".to_owned()
									})
								}
							},
							DataKind::Null => Err(Error {
								kind: ErrorKind::OperationError,
								message: format!("{} operator is not supported for type: Null.",
									operator
								)
							})
						}
					},
					_ => Err(Error {
						kind: ErrorKind::InternalError,
						message: format!("Unsupported unary operator '{}'.",
							operator
						)
					})
				}
			}
		}
	}

	pub fn run(&mut self, frame: &str, program: &ast::Program) {
		let counter = self.counter;
		self.counter = 0;
		self.stack.push(Frame::new(frame));

		while self.counter < program.statements.len() {
			match self.evalute_statement(
				program.statements
					.get(self.counter)
					.unwrap()
			) {
				Ok(_) => (),
				Err(error) => {
						eprintln!("Frame '{}',\nStatement {},\n{}", frame, self.counter, error);
						process::exit(1);
				}
			}
			self.counter += 1;
		}

		self.stack.pop();
		self.counter = counter;
	}
}