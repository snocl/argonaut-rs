use std::fmt;

/// A single name of a flag.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlagName<'a> {
    Short(char),
    Long(&'a str),
}

impl<'a> fmt::Display for FlagName<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FlagName::Short(short) => write!(f, "-{}", short),
            FlagName::Long(long) => write!(f, "--{}", long),
        }
    }
}

/// The name of an optional flag.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OptName<'a> {
    Normal(&'a str),
    NormalAndShort(&'a str, char),
}

impl<'a> OptName<'a> {
    /// Returns the long name of this optional argument
    pub fn name(&self) -> &'a str {
        match *self {
            OptName::Normal(name) | OptName::NormalAndShort(name, _) => name,
        }
    }
}
