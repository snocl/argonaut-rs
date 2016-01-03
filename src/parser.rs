use std::env;
use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum FlagName<'a> {
    Short(char),
    Long(&'a str),
}

impl<'a> fmt::Display for FlagName<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FlagName::Short(ch) => write!(f, "-{}", ch),
            FlagName::Long(flag) => write!(f, "--{}", flag),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum OptName<'a> {
    Short(char),
    Long(&'a str),
    ShortAndLong(char, &'a str),
}

impl<'a> OptName<'a> {
    fn flag_names(&self) -> Vec<FlagName<'a>> {
        use self::FlagName::*;
        match self {
            &OptName::Short(ch) => vec![Short(ch)],
            &OptName::Long(long) => vec![Long(long)],
            &OptName::ShortAndLong(ch, long) => vec![Short(ch), Long(long)],
        }
    } 
}

#[derive(Debug)]
enum ArgType<'a> {
    Single(&'a str),
    ZeroPlus(&'a str),
    OnePlus(&'a str),
    OptSingle(OptName<'a>),
    OptZeroPlus(OptName<'a>),
    OptOnePlus(OptName<'a>),
    Flag(OptName<'a>),
    InterruptFlag(OptName<'a>),
}

#[derive(Debug, Clone)]
enum OptType {
    Single,
    ZeroPlus,
    OnePlus,
}

#[derive(Debug)]                                                                                                                                                                                                                                                                                                                  
enum ReqType {
    ZeroPlus,
    OnePlus,
}

/// An argument given by the user
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

/// The result of a succesful parse. Either all the arguments are parsed and
/// bound, or an interrupt flag is encountered and the handle of the
/// corresponding argument is returned.
#[derive(Debug)]
pub enum ParseStatus<'a> {
    Parsed(ParsedArgs<'a>),
    Interrupted(OptName<'a>),
}

/// The result of a parse attempt.
pub type ParseResult<'a> = Result<ParseStatus<'a>, String>;

#[derive(Debug)]
pub struct Arg<'a> {
    argtype: ArgType<'a>,
    param: Option<&'a str>,
    help: Option<&'a str>,
}

impl<'a> Arg<'a> {
    pub fn help(mut self, help: &'a str) -> Self {
        self.help = Some(help);
        self
    } 
}

#[derive(Debug)]
pub enum ArgSpec<'a> {
    Req(&'a str),
    Opt(OptName<'a>),
}

pub mod arg {
    use super::{ArgSpec, OptName};
    pub fn req<'a>(name: &'a str) -> ArgSpec<'a> {
        ArgSpec::Req(name)
    }
    
    pub fn opt_short<'a>(name: char) -> ArgSpec<'a> {
        ArgSpec::Opt(OptName::Short(name))
    }
    
    pub fn opt_long<'a>(name: &'a str) -> ArgSpec<'a> {
        ArgSpec::Opt(OptName::Long(name))
    }
    
    pub fn opt_short_and_long<'a>(short: char, long: &'a str) -> ArgSpec<'a> {
        ArgSpec::Opt(OptName::ShortAndLong(short, long))
    }
}

impl<'a> ArgSpec<'a> {
    pub fn single(self, param: Option<&'a str>) -> Arg<'a> {
        let argtype = match self {
            ArgSpec::Req(name) => ArgType::Single(name),
            ArgSpec::Opt(name) => ArgType::OptSingle(name),
        };
        Arg { 
            argtype: argtype, param: param, help: None
        }
    }
    
    pub fn one_or_more(self, param: Option<&'a str>) -> Arg<'a> {
        let argtype = match self {
            ArgSpec::Req(name) => ArgType::OnePlus(name),
            ArgSpec::Opt(name) => ArgType::OptOnePlus(name),
        }; 
        Arg {
            argtype: argtype, param: param, help: None
        }
    }
    
    pub fn zero_or_more(self, param: Option<&'a str>) -> Arg<'a> {
        let argtype = match self {
            ArgSpec::Req(name) => ArgType::ZeroPlus(name),
            ArgSpec::Opt(name) => ArgType::OptZeroPlus(name),
        };
        Arg {
            argtype: argtype, param: param, help: None
        }
    }
    
    pub fn flag(self) -> Arg<'a> {
        let argtype = match self {
            ArgSpec::Opt(name) => ArgType::Flag(name),
            _ => panic!("Flags have to be optional!"),
        };
        Arg {
            argtype: argtype, param: None, help: None
        }
    }
    
    pub fn interrupt_flag(self) -> Arg<'a> {
        let argtype = match self {
            ArgSpec::Opt(name) => ArgType::InterruptFlag(name),
            _ => panic!("Flags have to be optional!"),
        };
        Arg {
            argtype: argtype, param: None, help: None
        }
    }
}

/// Creates an argument name (fat pointer) to the given argument if it is 
/// valid as such.
fn create_argument_name<'a>(arg: &'a str) -> GivenArgument<'a> {
    use self::GivenArgument::*;
    use self::FlagName::*;
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

#[derive(Debug)]
pub struct ParsedArgs<'a> {
    flags: HashMap<OptName<'a>, bool>,
}

#[derive(Debug)]
pub struct Parser<'title, 'a> {
    title: &'title str,
    positional: Vec<&'a str>,
    trail: Option<(&'a str, ReqType)>,
    options: HashMap<OptName<'a>, OptType>,
    flags: HashSet<OptName<'a>>,
    interrupt_flags: HashSet<OptName<'a>>,
    used_flags: HashSet<FlagName<'a>>,
    aliases: HashMap<FlagName<'a>, OptName<'a>>,
}

impl<'title, 'a> Parser<'title, 'a> {
    pub fn new(title: &'title str) -> Self {
        Parser {
            title: title,
            positional: Vec::new(),
            trail: None,
            options: HashMap::new(),
            flags: HashSet::new(),
            interrupt_flags: HashSet::new(),
            used_flags: HashSet::new(),
            aliases: HashMap::new(),
        }
    }
    
    pub fn add(&mut self, arg: &Arg<'a>) -> Result<(), String> {
        use self::ArgType::*;
        let opt_optname = match arg.argtype {
              OptSingle(ref optname) 
            | OptZeroPlus(ref optname) 
            | OptOnePlus(ref optname) 
            | Flag(ref optname) 
            | InterruptFlag(ref optname) => Some(optname),
            _ => None,
        };
        
        if let Some(optname) = opt_optname {
            let names = optname.flag_names();
            
            for name in names.iter() {
                if self.used_flags.contains(name) {
                    return Err(format!("The flag '{}' is already defined", name));
                }
            }
            
            for name in names.iter() {
                self.used_flags.insert(name.clone());
                self.aliases.insert(name.clone(), optname.clone());
            }
        }
        
        match arg.argtype {
            Single(name) => {
                self.positional.push(name);
            },
            ZeroPlus(name) => {
            
            },
            OnePlus(name) => {
            
            },
            Flag(ref optname) => {
                self.flags.insert(optname.clone());
            },
            InterruptFlag(ref optname) => {
                self.interrupt_flags.insert(optname.clone());
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
    
    /// Finds the alias of the name if any, or maps it to the other name type.
    fn convert_name(&self, name: &FlagName<'a>) -> OptName<'a> {
        if let Some(optname) = self.aliases.get(name) {
            optname.clone()
        } else {
            match name {
                &FlagName::Short(ch) => OptName::Short(ch),
                &FlagName::Long(long) => OptName::Long(long),
            }
        }
    }
    
    pub fn parse(&self, args: &Vec<&'a str>) -> ParseResult<'a> {
        use self::ParseStatus::{Parsed, Interrupted};
        use self::GivenArgument::{Value, Flag, ShortFlags};
        
        let arguments: Vec<_> = args.iter().map(
            |arg| create_argument_name(arg)).collect();
        
        //Check for interrupts first
        for arg in arguments.iter() {
            println!("Checking arg: {}...", arg);
            if let &Flag(ref name) = arg {
                let optname = self.convert_name(name);
                println!("Optname: {:?}", optname);
                if self.interrupt_flags.contains(&optname) {
                    return Ok(Interrupted(optname));
                }
            }
        }
            
        let mut flags: HashMap<OptName<'a>, bool> = self.flags.iter()
            .map(|f| (f.clone(), false)).collect();
            
        let parsed = ParsedArgs {
            flags: flags,
        };
        
        Ok(Parsed(parsed))
    }
}
