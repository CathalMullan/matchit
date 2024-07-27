#![deny(rust_2018_idioms, clippy::all)]

mod error;
mod router;
mod tree;

pub use error::{InsertError, MatchError};
pub use router::{Match, Param, Router};
