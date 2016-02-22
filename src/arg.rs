//! Parser arguments.
use common::OptName;

/// The different kinds of arguments that can be given to the parser.
#[derive(Debug, Clone, Copy)]
pub enum ArgType<'a> {
    Single(&'a str),
    ZeroPlus,
    OnePlus,
    OptSingle(OptName<'a>),
    OptZeroPlus(OptName<'a>),
    OptOnePlus(OptName<'a>),
    Switch(OptName<'a>),
    Interrupt(OptName<'a>),
    PassAlong(OptName<'a>),
}

/// An argument description for the parser.
#[derive(Debug, Clone, Copy)]
pub struct Arg<'a> {
    argtype: ArgType<'a>,
}

impl<'a> Arg<'a> {    
    /// Creates a positional argument with the given parameter name.
    pub fn positional(name: &'a str) -> Arg<'a> {
        Arg { argtype: ArgType::Single(name) }
    }
    
    /// Creates an argument that requires zero or more trailing parameters.
    pub fn optional_trail() -> Arg<'a> {
        Arg { argtype: ArgType::ZeroPlus }
    }
    
    /// Creates an argument that requires one or more trailing parameters.
    pub fn required_trail() -> Arg<'a> {
        Arg { argtype: ArgType::OnePlus }
    }
    
    /// Creates a new optional argument with a short name (ex 'h' for -h).
    pub fn named_and_short(name: &'a str, short: char) -> OptArg<'a> {
        OptArg { name: OptName::NormalAndShort(name, short) }
    }
    
    /// Creates a new optional argument with the given flag name.
    /// (ex "help" for --help).
    pub fn named(name: &'a str) -> OptArg<'a> {
        OptArg { name: OptName::Normal(name) }
    }
    
    /// Returns the option name of this argument. 
    /// This is the long name without prefixing dashes (eg: "help" for "--help").
    pub fn option_name(&self) -> Option<OptName<'a>> {
        use self::ArgType::*;
        match self.argtype {
              OptSingle(optname) 
            | OptZeroPlus(optname) 
            | OptOnePlus(optname) 
            | Switch(optname) 
            | Interrupt(optname) 
            | PassAlong(optname) => Some(optname),
            _ => None,
        }
    }
}

pub fn internal_get_raw<'a, 'b>(arg: &'b Arg<'a>) -> &'b ArgType<'a> {
    &arg.argtype
}

/// A partial builder for an optional argument.
#[derive(Debug)]
#[must_use]
pub struct OptArg<'a> {
    name: OptName<'a>,
}

impl<'a> OptArg<'a> {
    /// The argument takes a single parameter.
    pub fn single(self) -> Arg<'a> {
        Arg { argtype: ArgType::OptSingle(self.name) }
    }
    
    /// The argument takes one or more parameters.
    pub fn one_or_more(self) -> Arg<'a> {
        Arg { argtype: ArgType::OptOnePlus(self.name) }
    }
    
    /// The argument takes zero or more parameters.
    pub fn zero_or_more(self) -> Arg<'a> {
        Arg { argtype: ArgType::OptZeroPlus(self.name) }
    }
    
    /// The argument is an interrupt (the parse is interrupted when it is encountered).
    pub fn interrupt(self) -> Arg<'a> {
        Arg { argtype: ArgType::Interrupt(self.name) }
    }
    
    /// The argument is a switch (boolean flag).
    pub fn switch(self) -> Arg<'a> {
        Arg { argtype: ArgType::Switch(self.name) }
    }
    
    /// The argument is a passalong (all following arguments are collected)
    pub fn passalong(self) -> Arg<'a> {
        Arg { argtype: ArgType::PassAlong(self.name) }
    }
}