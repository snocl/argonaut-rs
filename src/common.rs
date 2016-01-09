// Created by Jakob Lautrup Nysom @ 05-01-2016

use std::fmt;
use std::collections::HashMap;

/// A single name of a flag.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FlagName<'a> {
    Short(char),
    Long(&'a str),
}
impl<'a> FlagName<'a> {
    /// Converts the name to a string (for error handling).
    pub fn to_string(&self) -> String {
        match self {
            &FlagName::Short(short) => format!("-{}", short),
            &FlagName::Long(long) => format!("--{}", long),
        }
    }
}

impl<'a> fmt::Display for FlagName<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FlagName::Short(ch) => write!(f, "-{}", ch),
            FlagName::Long(flag) => write!(f, "--{}", flag),
        }
    }
}

/// The name of an optional flag.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OptName<'a> {
    Short(char),
    Long(&'a str),
    ShortAndLong(char, &'a str),
}

impl<'a> OptName<'a> {
    /// Returns whether this flag is the given long option (ex "help" for --help).
    pub fn is_long(&self, other: &str) -> bool {
        use self::OptName::*;
        match self {
            &Long(long) | &ShortAndLong(_, long) => other == long,
            _ => false,
        }
    }
    
    /// Returns whether this flag is the given short option (ex 'h' for -h).
    pub fn is_short(&self, other: char) -> bool {
        use self::OptName::*;
        match self {
            &Short(short) | &ShortAndLong(short, _) => other == short,
            _ => false,
        }
    }
}

impl<'a> fmt::Display for OptName<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OptName::Short(short) => write!(f, "-{}", short),
            OptName::Long(long) => write!(f, "--{}", long),
            OptName::ShortAndLong(short, long) => {
                try!(write!(f, "-{}", short));
                write!(f, " | --{}", long)
            }
        }
    }
}

/// Finds the alias of the name if any, or maps it to the other name type.
pub fn convert_flag_name<'a>(aliases: &HashMap<FlagName<'a>, OptName<'a>>, 
        name: &FlagName<'a>) -> OptName<'a> {
    if let Some(optname) = aliases.get(name) {
        optname.clone()
    } else {
        match name {
            &FlagName::Short(ch) => OptName::Short(ch),
            &FlagName::Long(long) => OptName::Long(long),
        }
    }
}
