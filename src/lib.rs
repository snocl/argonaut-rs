//! Lets the user structure the arguments given to a program through a
//! command-line.

mod common;
mod arg;
mod parser;

pub use arg::{Arg, OptArg};
pub use parser::{Parser, Parse, StructuredArgument};