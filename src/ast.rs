pub enum AST {
	Expr(Expr),
	Program(Program),
	Statement(Statement),
	Value(Value)
}

pub enum Expr {
	BinaryOperation(crate::Operator, Box<AST>, Box<AST>),
	Value(Box<AST>)
}

pub struct Program {
	pub statements: Vec<AST>
}

pub struct Statement {
	pub index: u64,
	pub expr: Box<AST>
}

pub enum Value {
	Expr(Box<AST>),
	Integer(u64),
	Null,
	UnaryOperation(crate::Operator, Box<AST>)
}