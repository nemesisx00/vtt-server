#![allow(dead_code, non_snake_case, non_upper_case_globals)]

pub mod user;
pub mod token;

pub use user::Model as User;
pub use user::ActiveModel as UserActive;

pub use token::Model as Token;
pub use token::ActiveModel as TokenActive;
