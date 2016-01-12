// Created by Jakob Lautrup Nysom @ 05-01-2016

use std::collections::{HashMap, HashSet};
use std::fmt;
use common::{FlagName, OptName, convert_flag_name};
use parsed_args::ParsedArgs;

/// The different kinds of arguments that can be given to the parser.
#[derive(Debug, Clone)]
enum ArgType<'a> {
    Single(&'a str),
    ZeroPlus,
    OnePlus,
    OptSingle(OptName<'a>),
    OptZeroPlus(OptName<'a>),
    OptOnePlus(OptName<'a>),
    Switch(OptName<'a>),
    Interrupt(OptName<'a>),
}

/// An argument description for the parser. Use methods on 'arg' to create them.
#[derive(Debug)]
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
    
    /// Creates an argument requires one or more trailing parameters.
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
    pub fn option_name(&self) -> Option<&OptName<'a>> {
        use self::ArgType::*;
        match self.argtype {
              OptSingle(ref optname) 
            | OptZeroPlus(ref optname) 
            | OptOnePlus(ref optname) 
            | Switch(ref optname) 
            | Interrupt(ref optname) => Some(optname),
            _ => None,
        }
    }
}

/// The builder for an optional argument.
#[derive(Debug)]
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
}

/// The possible types of an optional argument.
#[derive(Debug, Clone)]
enum OptType {
    Single,
    ZeroPlus,
    OnePlus,
}

/// Returns the flag names that might denote this option.
fn optional_flag_names<'a>(name: &OptName<'a>) -> Vec<FlagName<'a>> {
    use common::FlagName::*;
    match name {
        &OptName::Normal(long) => vec![Long(long)],
        &OptName::NormalAndShort(long, ch) => vec![Short(ch), Long(long)],
    }
}

/// The possible types of a required argument that isn't positional.
#[derive(Debug)]                                                                                                                                                                                                                                                                                                                  
enum ReqType {
    ZeroPlus,
    OnePlus,
}

/// The result of a succesful parse. Either all the arguments are parsed and
/// bound, or an interrupt flag is encountered and the handle of the
/// corresponding argument is returned.
#[derive(Debug)]
pub enum ParseStatus<'a> {
    Parsed(ParsedArgs<'a>),
    Interrupted(&'a str),
}

/// An error found when attempting to parse a set of arguments.
#[derive(Debug)]
pub enum ParseError<'a> {
    /// The given argument is not recognized by the parser.
    UnknownArgument(String),
    /// The given short flag takes input and therefore cannot be grouped when
    /// used (if '-x' takes the argument 'FOO', you cannot call '-vasx').
    GroupedNonFlag(String),
    /// The argument is missing a parameter.
    MissingParameter(String),
    /// The positional argument with this name is missing.
    MissingArgument(&'a str),
    /// The required trail argument is missing.
    MissingTrail,
    /// The given positional arguments were not expected by the parser.
    UnexpectedArguments(Vec<&'a str>)
}

/// The result of a parse attempt.
pub type ParseResult<'a> = Result<ParseStatus<'a>, ParseError<'a>>;

/// An argument given by the user.
#[derive(Debug)]
enum GivenArgument<'a> {
    Value(&'a str),
    Flag(FlagName<'a>),
    ShortFlags(Vec<FlagName<'a>>),
}

impl<'a> fmt::Display for GivenArgument<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GivenArgument::Value(value) => write!(f, "{}", value),
            GivenArgument::Flag(ref flag) => write!(f, "{}", flag),
            GivenArgument::ShortFlags(ref flags) => {
                try!(write!(f, "-"));
                for flag in flags.iter() {
                    if let &FlagName::Short(ch) = flag {
                        try!(write!(f, "{}", ch));
                    }
                }
                Ok(())
            }
        }
    }
}

/// Creates an argument name (fat pointer) to the given argument if it is 
/// valid as such.
fn create_argument_name<'a>(arg: &'a str) -> GivenArgument<'a> {
    use self::GivenArgument::*;
    use common::FlagName::*;
    if arg.starts_with("--") {
        Flag(Long(&arg[2..]))
        
    } else if arg.starts_with("-") {
        if arg.len() == 2 {
            Flag(Short(arg.chars().nth(1).unwrap()))
        } else {
            ShortFlags(arg.chars().skip(1).map(|ch| Short(ch)).collect())
        }
        
    } else {
        Value(arg)
    }
}

/// Attempts to find enough parameters for the given option type.
fn find_parameters<'a, 'b>(name: &FlagName<'b>, opt_type: &OptType,
        args: &[GivenArgument<'a>]) 
        -> Result<Vec<&'a str>, ParseError<'b>> {
    use self::ParseError::*;
    use self::GivenArgument::*;
    //println!("Finding parameters of {} ({:?}) in {:?}", name, opt_type, args);
    let mut params = Vec::new();
    match opt_type {
        &OptType::Single => {
            if args.len() < 1 {
                return Err(MissingParameter(name.to_string()));
            }
            if let Value(val) = args[0] {
                params.push(val);
            } else {
                return Err(MissingParameter(name.to_string()));
            }
        },
        &OptType::ZeroPlus => {
            params.extend(args.iter().take_while(|arg| {
                if let &&Value(_) = arg {
                    true
                } else {
                    false
                }
            }).map(|arg| {
                if let &Value(val) = arg {
                    val
                } else {
                    panic!("take_while invariant broken!");
                }
            }));
        },
        &OptType::OnePlus => {
            if args.len() < 1 {
                return Err(MissingParameter(name.to_string()));
            }
            if let Value(val) = args[0] {
                params.push(val);
            } else {
                return Err(MissingParameter(name.to_string()));
            }
            params.extend(args.iter().skip(1).take_while(|arg| {
                if let &&Value(_) = arg {
                    true
                } else {
                    false
                }
            }).map(|arg| {
                if let &Value(val) = arg {
                    val
                } else {
                    panic!("take_while invariant broken!");
                }
            }));
        },
    }
    Ok(params)
}

/// An argument parser.
#[derive(Debug)]
pub struct Parser<'a> {
    positional: Vec<&'a str>,
    trail: Option<ReqType>,
    options: HashMap<OptName<'a>, OptType>,
    switches: HashSet<OptName<'a>>,
    interrupts: HashSet<OptName<'a>>,
    used_flags: HashSet<FlagName<'a>>,
    aliases: HashMap<FlagName<'a>, OptName<'a>>,
}

impl<'a> Parser<'a> {
    /// Creates a new parser with the given title.
    pub fn new() -> Self {
        Parser {
            positional: Vec::new(),
            trail: None,
            options: HashMap::new(),
            switches: HashSet::new(),
            interrupts: HashSet::new(),
            used_flags: HashSet::new(),
            aliases: HashMap::new(),
        }
    }
    
    /// Attempts to add an argument to this parser.
    pub fn add(&mut self, arg: &Arg<'a>) -> Result<(), String> {
        use self::ArgType::*;
      
        if let Some(optname) = arg.option_name() {
            let names = optional_flag_names(&optname);
            
            for name in names.iter() {
                if self.used_flags.contains(name) {
                    return Err(format!(
                        "The flag '{}' is already defined", name
                    ));
                }
            }
            
            for name in names.iter() {
                self.used_flags.insert(name.clone());
                self.aliases.insert(name.clone(), optname.clone());
            }
        }
        
        match arg.argtype {
            Single(name) => {
                if self.positional.contains(&name) {
                    return Err(format!(
                        "A positional argument with the name '{}' has already \
                        been added", name
                    ));
                } else {
                    self.positional.push(name);
                }
            },
            ZeroPlus => {
                if self.trail.is_some() {
                    return Err(format!(
                        "A trailing argument has already been set",
                    ));
                }
                self.trail = Some(ReqType::ZeroPlus)
            },
            OnePlus => {
                if self.trail.is_some() {
                    return Err(format!(
                        "A trailing argument has already been set",
                    ));
                }
                self.trail = Some(ReqType::OnePlus)
            
            },
            Switch(ref optname) => {
                self.switches.insert(optname.clone());
            },
            Interrupt(ref optname) => {
                self.interrupts.insert(optname.clone());
            },
              OptSingle(ref optname) 
            | OptZeroPlus(ref optname) 
            | OptOnePlus(ref optname) => {
                let opt_type = match arg.argtype {
                    OptSingle(_) => OptType::Single,
                    OptZeroPlus(_) => OptType::ZeroPlus,
                    OptOnePlus(_) => OptType::OnePlus,
                    _ => unreachable!(),
                };
                self.options.insert(optname.clone(), opt_type);
            },
        }
        Ok(())
    }
    
    /// Attempts to parse the given arguments with this parser.
    pub fn parse(&self, args: &Vec<&'a str>) -> ParseResult<'a> {
        use self::ParseStatus::{Parsed, Interrupted};
        use self::GivenArgument::{Value, Flag, ShortFlags};
        use self::ParseError::*;
        
        let arguments: Vec<_> = args.iter().map(
            |arg| create_argument_name(arg)).collect();
        
        // Check for interrupts first
        for arg in arguments.iter() {
            //println!("Checking arg: {}...", arg);
            if let &Flag(ref name) = arg {
                let optname = convert_flag_name(&self.aliases, name);
                //println!("Optname: {:?}", optname);
                if self.interrupts.contains(&optname) {
                    return Ok(Interrupted(optname.long()));
                }
                
                // SAFE_FLAGS: Validate that all flags are valid
                if ! self.used_flags.contains(name) {
                    return Err(UnknownArgument(name.to_string()));
                }
            }
        }
            
        let mut switches: HashMap<OptName<'a>, bool> = self.switches.iter()
            .map(|f| (f.clone(), false)).collect();
        
        let mut positional = Vec::new();
        let mut multiples = HashMap::new();
        let mut singles = HashMap::new();
        
        // Check the general arguments in order
        let mut index = 0;
        while index < args.len() {
            match arguments[index] {
                Value(val) => {
                    positional.push(val);
                    index += 1;
                },
                
                Flag(ref flag_name) => {
                    let opt_name = convert_flag_name(&self.aliases, flag_name);
                    if self.switches.contains(&opt_name) {
                        switches.insert(opt_name, true);
                        index += 1;
                    } else {
                        // Due to the SAFE_FLAGS invariant, this is safe
                        let opt_type = self.options.get(&opt_name)
                            .expect("invariant: SAFE_FLAGS");
                        
                        index += 1;
                        let params = try!(find_parameters(
                            flag_name, opt_type, &arguments[index..]
                        ));
                        //println!("Params for {}: {:?}", &flag_name, &params);
                        match opt_type {
                            &OptType::Single => {
                                index += 1;
                                singles.insert(opt_name, Some(params[0]));
                            },
                            &OptType::ZeroPlus => {
                                index += params.len();
                                multiples.insert(opt_name, Some(params));
                            },
                            &OptType::OnePlus => {
                                index += params.len();
                                multiples.insert(opt_name, Some(params));
                            },
                        }
                    }
                },
                ShortFlags(ref flag_names) => {
                    for flag_name in flag_names.iter() {
                        let opt_name = convert_flag_name(&self.aliases, flag_name);
                        if self.switches.contains(&opt_name) {
                            switches.insert(opt_name, true);
                        } else {
                            return Err(GroupedNonFlag(flag_name.to_string()));
                        }
                    }
                    index += 1;
                },
            }
        }
        
        // Ensure that enough positional arguments are given
        
        
        // Fewer than the positional arguments
        let positions = self.positional.len();
        if positional.len() < positions {
            return Err(MissingArgument(self.positional[positional.len()]));
            
        // Only the positional arguments
        } else if positional.len() == positions {
            if let Some(ReqType::OnePlus) = self.trail {
                return Err(MissingTrail);
            }
        
        // More than the positional arguments
        } else {
            if let None = self.trail {
                let mut unexpected = Vec::new();
                unexpected.extend(positional[positions..].iter());
                return Err(UnexpectedArguments(unexpected))
            }
        }
        
        let mut positional_args: HashMap<&str, &str> = HashMap::new();
        for (num, posname) in self.positional.iter().enumerate() {
            positional_args.insert(posname, positional[num]);
        }
        
        let trail = if self.trail.is_some() {
            Some(positional[positions..].iter().map(|s| *s).collect::<Vec<_>>())
        } else {
            None
        };
        
        let parsed = ParsedArgs::new(
            positional_args, 
            trail,
            singles, 
            multiples, 
            switches, 
            self.aliases.clone()
        );
        
        Ok(Parsed(parsed))
    }
}