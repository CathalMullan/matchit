#![deny(rust_2018_idioms, clippy::all)]

pub mod error;
pub mod router;
pub mod tree;

pub use error::{InsertError, MatchError};
pub use router::{Match, Param, Router};
