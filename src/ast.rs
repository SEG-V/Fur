pub enum Expr {
	BinaryOperation(crate::Operator, Box<Expr>, Box<Expr>),
	Value(Value)
}

pub struct Program {
	pub statements: Vec<Statement>
}

pub struct Statement {
	pub expr: Expr
}

pub enum Value {
	Expr(Box<Expr>),
	Integer(u64),
	Null,
	UnaryOperation(crate::Operator, Box<Value>)
}