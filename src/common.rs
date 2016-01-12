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
    Normal(&'a str),
    NormalAndShort(&'a str, char),
}

impl<'a> OptName<'a> {
    /// Returns the long name of this optional argument
    pub fn long(&self) -> &'a str {
        match self {
            &OptName::Normal(long) | &OptName::NormalAndShort(long, _) => long,
        }
    }
}

impl<'a> fmt::Display for OptName<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OptName::Normal(name) => write!(f, "--{}", name),
            OptName::NormalAndShort(name, short) => {
                try!(write!(f, "-{}", short));
                write!(f, " | --{}", name)
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
            // NOTE: This is basically always wrong, but doesn't harm anyone
            &FlagName::Short(ch) => OptName::NormalAndShort("", ch),
            &FlagName::Long(long) => OptName::Normal(long),
        }
    }
}
