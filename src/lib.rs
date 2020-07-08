//! This crate provides a string type that can have
//! [ruby glosses](https://en.wikipedia.org/wiki/Ruby_character)
//! attached to parts of it. (If you're not familiar with the concept of Ruby characters, follow
//! that link right there first.)
//!
//! ## Maturity
//!
//! This crate is a "Minimum Viable Product" right now, so the API is quite bare. Feel free to
//! create a GitHub issue if you're missing an API for your usecase. Better yet, send a PR to add
//! your desired API. (For bigger changes, please create an issue beforehand to discuss the
//! API design.)

mod iterator;
pub use iterator::*;
mod string;
pub use string::*;
