// Created by Jakob Lautrup Nysom @ 05-01-2016
//! Lets the user structure the arguments given to a program through a
//! command-line.

mod common;
mod parsed_args;
mod parser;

pub use common::OptName;
pub use parser::{Parser, Arg, ParseStatus, OptArg};
pub use parsed_args::{ParsedArgs, ParsedArgsAccess};