#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

pub mod user;
pub mod token;

pub use user::Model as User;
pub use user::ActiveModel as UserActive;

pub use token::Model as Token;
pub use token::ActiveModel as TokenActive;
