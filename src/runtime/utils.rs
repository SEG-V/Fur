use std::fmt;
use std::ops;

pub trait Data {
	fn kind(&self) -> DataKind;
	fn value(&self) -> DataValue;
}

pub enum DataKind {
	Integer,
	Null
}

pub enum DataValue {
	I64(i64),
	Null,
	U64(u64)
}

pub struct Error {
	pub kind: ErrorKind,
	pub message: String
}

pub enum ErrorKind {
	InternalError,
	OperationError
}

pub struct Frame {
	pub name: String,
	pub variables: Vec<Box<dyn Data>>
}

pub struct Integer<I>
{
	pub value: I
}

pub struct Null {/* Empty Struct */}

impl fmt::Display for DataKind {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Integer => write!(f, "Integer"),
			Self::Null => write!(f, "Null")
		}
	}
}

impl fmt::Display for DataValue {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::I64(value) => write!(f, "{}", value),
			Self::Null => write!(f, "Null"),
			Self::U64(value) => write!(f, "{}", value)
		}
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}: {}", match &self.kind {
			ErrorKind::InternalError => "InternalError",
			ErrorKind::OperationError => "OperationError"
		}, self.message)
	}
}

impl Frame {
	pub fn new(name: &str) -> Self {
		Self {
			name: name.to_owned(),
			variables: Vec::new()
		}
	}
}

impl Data for Integer<i64> {
	fn kind(&self) -> DataKind {
		DataKind::Integer
	}

	fn value(&self) -> DataValue {
		DataValue::I64(self.value)
	}
}

impl Data for Integer<u64> {
	fn kind(&self) -> DataKind {
		DataKind::Integer
	}

	fn value(&self) -> DataValue {
		DataValue::U64(self.value)
	}
}

impl Data for Null {
	fn kind(&self) -> DataKind {
		DataKind::Null
	}

	fn value(&self) -> DataValue {
		DataValue::Null
	}
}

pub fn add<T>(x: T, y: T) -> T::Output
where T: ops::Add {
	x + y
}

pub fn div<T>(x: T, y: T) -> T::Output
where T: ops::Div {
	x / y
}

pub fn mul<T>(x: T, y: T) -> T::Output
where T: ops::Mul {
	x * y
}

pub fn sub<T>(x: T, y: T) -> T::Output
where T: ops::Sub {
	x - y
}