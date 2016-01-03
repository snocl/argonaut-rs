// Created by Jakob Lautrup Nysom @ 31-12-2015

use std::collections::{HashMap, HashSet};
use std::iter::Iterator;
use std::fmt;

type Id = usize;

// Global id to help uniquely identify parsers and avoid misuse of argument
// handles with the output of other parsers.
// TODO: Find a thread-safe alternative to this, or use the panicky way?
//static mut PARSER_ID: Id = 1;


// =============================================================================
// ============================ Parsed arguments ===============================
// =============================================================================

#[derive(Debug)]
pub struct ParsedArguments<'a> {
    req_singles: HashMap<Id, &'a str>,
    req_vararg: Option<(Id, Vec<&'a str>)>,
    opt_singles: HashMap<Id, Option<&'a str>>,
    opt_multiples: HashMap<Id, Option<Vec<&'a str>>>,
    opt_flags: HashMap<Id, bool>,
}

impl<'a> ParsedArguments<'a> {
    fn get_single_required(&self, id: &Id) -> &'a str {
        self.req_singles.get(id).unwrap_or_else(|| panic!(format!(
            "No required single argument found with id {}", id
        )))
    }
    
    fn get_multiple_required(&'a self, id: &Id) -> &'a Vec<&'a str> {
        if let Some((ref vid, ref args)) = self.req_vararg {
            if vid == id {
                args
            } else {
                panic!(format!(
                    "The given tag has the wrong id for the \
                    multiple-parameter argument ({} != {})!", id, vid
                ))
            }
        } else {
            panic!("No multiple-parameter argument was defined");
        }
    }
    
    fn get_single_optional(&'a self, id: &Id) -> &'a Option<&'a str> {
        self.opt_singles.get(id).unwrap_or_else(|| panic!(format!(
            "No optional single-parameter argument found with id {}", id
        )))
    }
    
    fn get_multiple_optional(&'a self, id: &Id) -> &'a Option<Vec<&'a str>> {
        self.opt_multiples.get(id).unwrap_or_else(|| panic!(format!(
            "No optional multiple-parameter argument found with id {}", id
        )))
    }
    
    fn get_flag(&self, id: &Id) -> bool {
        *self.opt_flags.get(id).unwrap_or_else(|| panic!(format!(
            "No flag found with id {}", id
        )))
    }
}

/// The result of a succesful parse. Either all the arguments are parsed and
/// bound, or an interrupt flag is encountered and the handle of the
/// corresponding argument is returned.
pub enum ParseStatus<'a> {
    Parsed(ParsedArguments<'a>),
    Interrupted(InterruptFlagArgumentTag),
}

/// The result of a parse attempt.
pub type ParseResult<'a> = Result<ParseStatus<'a>, String>;

/// The name of an argument.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum ArgumentName<'a> {
    Short(char),
    Long(&'a str),
}

impl<'a> fmt::Display for ArgumentName<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ArgumentName::Short(ch) => write!(f, "-{}", ch),
            ArgumentName::Long(flag) => write!(f, "--{}", flag),
        }
    }
}

/// An argument given by the user
enum GivenArgument<'a> {
    Name(ArgumentName<'a>),
    MultipleShort(Vec<ArgumentName<'a>>),
    Value(&'a str),
}

/// An argument that takes multiple parameters.
#[derive(Debug, Clone)]
enum MultipleArguments {
    ZeroOrMore,
    OneOrMore,
}

/// The possible types of arguments.
#[derive(Debug, Clone)]
enum ArgumentType {
    SingleRequired,
    MultipleRequired(MultipleArguments),
    SingleOptional,
    MultipleOptional,
    Flag,
}

/// The full description of an argument.
#[derive(Debug, Clone)]
struct FullDescription<'a> {
    names: Vec<ArgumentName<'a>>,
    argtype: ArgumentType,
    param_name: Option<&'a str>,
    help: Option<&'a str>,
}

impl <'a> FullDescription<'a> {
    /// Creates a new description.
    fn new(names: Vec<ArgumentName<'a>>, param_name: Option<&'a str>, 
            argtype: ArgumentType, help: Option<&'a str>) 
            -> FullDescription<'a> {
        FullDescription {
            names: names,
            argtype: argtype,
            param_name: param_name,
            help: help,
        }
    }
}

/// The handle for an interrupt flag argument.
#[derive(Debug, Clone, PartialEq)]
pub struct InterruptFlagArgumentTag {
    id: Id
}

/// The name of an optional argument.
#[derive(Debug, Clone)]
enum OptionalName<'a> {
    Short(char),
    Long(&'a str),
    ShortAndLong(char, &'a str),
}
impl <'a> OptionalName<'a> {
    pub fn create_names(&'a self) -> Vec<ArgumentName<'a>> {
        use self::ArgumentName::*;
        match *self {
            OptionalName::Short(ch) => vec![Short(ch)],
            OptionalName::Long(long) => vec![Long(long)],
            OptionalName::ShortAndLong(ch, long) => vec![Short(ch), Long(long)],
        }
    }
}

// Declare and implement all the handles and their 'get' methods.
tag_structs! {
    SingleRequiredTag: get_single_required -> &'a str,
    MultipleRequiredTag: get_multiple_required -> &Vec<&'a str>,
    SingleOptionalTag: get_single_optional -> &Option<&'a str>,
    MultipleOptionalTag: get_multiple_optional -> &Option<Vec<&'a str>>,
    FlagTag: get_flag -> bool
}

// =============================================================================
// ======================== Argument count builders ============================
// =============================================================================

argument_type_structs! {
    common: Required {
        name: &'a str,
        help: Option<&'a str>
    }
    
    SingleRequiredArgument {}
    tag: add_single_required -> SingleRequiredTag,
    
    constructors: {
        single() -> {}
    }
    
    MultipleRequiredArguments {
        argtype: MultipleArguments
    }
    tag: add_multiple_required -> MultipleRequiredTag,
    
    constructors: {
        
    }
}

impl<'a> Required<'a> {
    /// Creates a bulider for a required argument.
    pub fn new(name: &'a str) -> Required<'a> {
        Required {
            name: name,
            help: None,
        }
    }
}

argument_type_structs! {
    common: Optional {
        name: OptionalName<'a>,
        help: Option<&'a str>
    }
    
    InterruptFlagArgument {}
    tag: add_interrupt_flag -> InterruptFlagArgumentTag,
    
    constructors: {
        interrupt() -> {}
    }
    
    FlagArgument {}
    tag: add_flag -> FlagTag,
    
    constructors: {
        flag() -> {}
    }
    
}

impl<'a> Optional<'a> {
    /// Creates a builder for an optional argument with the given short name
    /// prefixed by '-' (eg '-a').
    pub fn short(name: char) -> Optional<'static> {
        Optional {
            name: OptionalName::Short(name),
            help: None,
        }
    }
    
    /// Creates a builder for an optional argument with the given long name
    /// prefixed by '--' (eg '--all').
    pub fn long(name: &'a str) -> Optional<'a> {
        Optional {
            name: OptionalName::Long(name),
            help: None,
        }
    }
    
    /// Creates a builder for an optional argument with the given short and
    /// long names, where the short name is prefixed by '-' and the long name by 
    /// '--'.
    pub fn short_and_long(short_name: char, long_name: &'a str) 
            -> Optional<'a> {
        Optional {
            name: OptionalName::ShortAndLong(short_name, long_name),
            help: None,
        }
    }
}

// =============================================================================
// ================================= Parser ====================================
// =============================================================================

/// Creates the description strings for an argument with the given 
/// names, parameter name and description.
fn create_description(names: &Vec<String>, argname: &str, description: &str) 
        -> (String, String) {
    ("".into(), "".into())
}

/// Creates an argument name (fat pointer) to the given argument if it is 
/// valid as such.
fn create_argument_name<'a>(arg: &'a str) -> GivenArgument<'a> {
    use self::GivenArgument::*;
    if arg.starts_with("--") {
        match arg.char_indices().nth(2).map(|(idx, _)| idx) {
            Some(index) => Name(ArgumentName::Long(&arg[index..])),
            None => Name(ArgumentName::Long("")),
        }
    
    } else if arg.starts_with("-") {
        if arg.len() == 2 {
            let ch = arg.char_indices().nth(1).map(|(_, ch)| ch).unwrap();
            Name(ArgumentName::Short(ch))
        } else {
            MultipleShort(arg.chars().skip(1).map(|ch| ArgumentName::Short(ch))
                .collect())
        }
    
    } else {
        Value(arg)
    }
}

#[derive(Debug, Clone)]
pub struct ArgumentParser<'a, 'title> {
    pub title: &'title str,
    next_id: Id,
    req_singles: Vec<Id>,
    req_vararg: Option<(Id, MultipleArguments)>,
    opt_singles: HashMap<ArgumentName<'a>, Id>,
    opt_multiples: HashMap<ArgumentName<'a>, (Id, MultipleArguments)>,
    opt_flags: HashMap<ArgumentName<'a>, Id>,
    interrupt_flags: HashMap<ArgumentName<'a>, InterruptFlagArgumentTag>,
    taken_names: HashSet<ArgumentName<'a>>,
    req_descriptions: Vec<FullDescription<'a>>,
    opt_descriptions: Vec<FullDescription<'a>>,
}

impl<'a, 'title> ArgumentParser<'a, 'title> {
    /// Creates a new argument parser with the given title.
    pub fn new(title: &'title str) -> Self {
        ArgumentParser {
            title: title,
            next_id: 1,
            req_singles: Vec::new(),
            req_vararg: None,
            opt_singles: HashMap::new(),
            opt_multiples: HashMap::new(),
            opt_flags: HashMap::new(),
            interrupt_flags: HashMap::new(),
            taken_names: HashSet::new(),
            req_descriptions: Vec::new(),
            opt_descriptions: Vec::new(),
        }
    }
    
    /// Generates the next argument id for the parser.
    fn generate_id(&mut self) -> Id {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
    
    /// Checks that the given names are not registered with the parser.
    fn check_names(&self, names: &Vec<ArgumentName>) -> Result<(), String> {
        for name in names {
            if self.taken_names.contains(name) {
                return Err(format!("The argument '{}' is already taken!",
                    name
                ));
            }
        }
        Ok(())
    }
    
    /// Attempts to add a required single-parameter argument.
    fn add_single_required(&mut self, arg: &'a SingleRequiredArgument<'a>) 
            -> Result<SingleRequiredTag, String> {
        if self.req_vararg.is_some() {
            Err(format!(
                "Could not add the argument '{}', since all required \
                single-parameter arguments must be added before the \
                variable-parameter argument", arg.name
            ))
        } else {
            let id = self.generate_id();
            //self.req_descriptions.push(create)
            self.req_singles.push(id);
            Ok( SingleRequiredTag { id: id } )
        }
    }
    
    /// Attempts to add a required multiple-parameter argument.
    fn add_multiple_required(&mut self, arg: &'a MultipleRequiredArguments<'a>)
            -> Result<MultipleRequiredTag, String> {
        if self.req_vararg.is_some() {
            Err(String::from(
                "A required multi-count argument is already defined\
            "))
        } else {
            let id = self.generate_id();
            self.req_vararg = Some((id, arg.argtype.clone()));
            Ok( MultipleRequiredTag { id: id } )
        }
    }
    
    /// Attempts to add an interrupt flag.
    fn add_interrupt_flag(&mut self, flag: &'a InterruptFlagArgument<'a>)
            -> Result<InterruptFlagArgumentTag, String> {
        let names = flag.name.create_names();
        try!(self.check_names(&names));
        let id = self.generate_id();
        let tag = InterruptFlagArgumentTag { id: id };
        for name in names {
            self.interrupt_flags.insert(name.clone(), tag.clone());
            self.taken_names.insert(name);
        }
        Ok(tag)
    }
    
    /// Attempts to add a flag.
    fn add_flag(&mut self, flag: &'a FlagArgument<'a>) 
            -> Result<FlagTag, String> {
        let names = flag.name.create_names();
        try!(self.check_names(&names));
        let id = self.generate_id();
        for name in names {
            self.opt_flags.insert(name.clone(), id);
            self.taken_names.insert(name);
        }
        Ok( FlagTag { id: id } )
    }
    
    /// Returns a result indicating whether the parser takes the given argument.
    pub fn check_argument(&self, argument: &ArgumentName<'a>)
            -> Result<(), String> {
        if ! self.taken_names.contains(argument) {
            return Err(format!(
                "Unrecognized argument: '{}'", argument
            ))
        }
        Ok(())
    }
    
    /// Parses the given arguments or returns an error if they do not satisfy
    /// the declared arguments of the parser.
    pub fn parse(&self, args: &[&str] )
            -> ParseResult<'a> {
       
        use self::ParseStatus::{Parsed, Interrupted};
        use self::GivenArgument::{Name, Value, MultipleShort};
        
        let arguments: Vec<_> = args.iter().map(
            |arg| create_argument_name(arg)).collect();
        
        // Check for interrupts first
        for arg in arguments.iter() {
            if let &Name(ref name) = arg {
                if let Some(tag) = self.interrupt_flags.get(name) {
                    return Ok(Interrupted(tag.clone()))
                }
            }
        }
        
        // TODO: Consider whether duplicate arguments given should be warned
        // by default, or only with a specific flag set.
        // Keep track of the set ids, and which parameter this was from
        //let mut found: HashMap<Id, String> = HashMap::new();
        
        // If it is not interrupted, prepare the structures        
        let mut req_singles = HashMap::new();
        let mut req_vararg = None;
        let mut opt_singles: HashMap<Id, Option<&'a str>> = 
            self.opt_singles.values()
            .map(|id| (*id, None)).collect();
        
        let mut opt_multiples: HashMap<Id, Option<Vec<&'a str>>> =
            self.opt_multiples.values()
            .map(|&(ref id, ref args)| (*id, None)).collect();
        
        let mut opt_flags: HashMap<Id, bool> = 
            self.opt_flags.values()
            .map(|id| (*id, false)).collect();
        
        let mut i = 0;
        while i < arguments.len() {
            let ref arg = arguments[i];
            
            match arg {
                &Value(ref val) => { i += 1; },
                
                &Name(ref name) => {
                    try!(self.check_argument(name));
                    if let Some(id) = self.opt_flags.get(name) {
                        opt_flags.insert(*id, true);
                    } else {
                        return Err(format!(
                            "Unrecognized flag '{}'", name
                        ));
                    }
                    i += 1;
                },
                
                &MultipleShort(ref chars) => {
                    for arg in chars {
                        try!(self.check_argument(&arg));
                        if let Some(id) = self.opt_flags.get(&arg) {
                            opt_flags.insert(*id, true);
                        } else {
                            return Err(format!(
                                "The short argument '{}' is not a flag, and \
                                so cannot be grouped with other flags.\
                            ", arg));
                        }
                        i += 1;
                    }
                    
                }
            }
        }
        
        let mut parsed = ParsedArguments { 
            req_singles: req_singles,
            req_vararg: req_vararg,
            opt_singles: opt_singles,
            opt_multiples: opt_multiples,
            opt_flags: opt_flags,
        };
        
        for arg in args.iter() {
            println!("Parsing '{}'", arg);
        }
        
        Ok(Parsed(parsed))
    }
}