use std::fmt::{Display, Formatter};

use serde::de::StdError;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
	Message(String),
}

impl Display for Error {
	fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
		Ok(())
	}
}

impl StdError for Error {}

impl serde::de::Error for Error {
	fn custom<T>(msg: T) -> Self where T: Display {
		Error::Message(msg.to_string())
	}
}
