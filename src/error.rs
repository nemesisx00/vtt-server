#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::{
	error::Error,
	fmt::{
		Display,
		Formatter,
		Result,
	},
};

#[derive(Debug)]
pub struct MessageError
{
	pub message: String,
}

impl Error for MessageError { }

impl Display for MessageError
{
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		return write!(f, "{}", self.message);
	}
}
