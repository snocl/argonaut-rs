use std::fmt;

/// A single name of a flag.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OptName<'a> {
    Normal(&'a str),
    NormalAndShort(&'a str, char),
}

impl<'a> OptName<'a> {
    /// Returns the long name of this optional argument
    pub fn name(&self) -> &'a str {
        match self {
            &OptName::Normal(name) | &OptName::NormalAndShort(name, _) => name,
        }
    }
}
