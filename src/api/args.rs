#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::collections::HashMap;

pub fn parseArgs(args: String) -> HashMap<String, String>
{
	let mut data = HashMap::<String, String>::default();
	let pairs: Vec<&str> = args.split("&").collect();
	for pair in pairs
	{
		if let Some((key, value)) = pair.split_once("=")
		{
			data.insert(key.to_string(), value.to_string());
		}
	}
	
	return data;
}

#[cfg(test)]
mod tests
{
	#![allow(unused_imports)]
	use super::*;
	
	#[test]
	fn test_parseArgs()
	{
		let args = "one=1&two=2.0&three=three".to_owned();
		let mut expected = HashMap::<String, String>::default();
		expected.insert("one".to_owned().to_string(), "1".to_owned());
		expected.insert("two".to_owned(), "2.0".to_owned());
		expected.insert("three".to_owned(), "three".to_owned());
		
		let result = parseArgs(args);
		assert_eq!(result, expected);
	}
}
