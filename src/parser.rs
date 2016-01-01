// Created by Jakob Lautrup Nysom @ 31-12-2015

use std::collections::{HashMap, HashSet};
use std::iter::Iterator;

type Id = usize;

// Global id to help uniquely identify parsers and avoid misuse of argument
// handles with the output of other parsers.
// TODO: Find a thread-safe alternative to this, or use the panicky way?
//static mut PARSER_ID: Id = 1;


/// Metadata for the help options of an argument or flag.
#[derive(Debug, Clone)]
struct ArgumentDescription<'a> {
    name: Option<&'a str>,
    help: Option<&'a str>,
}

impl <'a> ArgumentDescription<'a> {
    /// Creates an empty argument description.
    fn empty() -> ArgumentDescription<'a> {
        ArgumentDescription {
            name: None,
            help: None,
        }
    }
}

/// The description of how to use a specific argument or flag.
#[derive(Debug)]
struct HelpDescription<'a> {
    usage: String,
    help: &'a str,
}

impl <'a> HelpDescription<'a> {
    fn new(usage: String, help: &'a str) -> HelpDescription<'a> {
        HelpDescription {
            usage: usage,
            help: help,
        }
    }
}

/// An argument that takes multiple parameters.
#[derive(Debug, Clone)]
enum MultipleArguments {
    Count(usize),
    ZeroOrMore,
    OneOrMore,
}

impl MultipleArguments {
    /// Creates a description of the parameters for a usage string.
    fn usage(&self, param_name: &str) -> String {
        match *self {
            MultipleArguments::Count(n) => {
                if n >= 4 {
                    format!("{0} {0}1 {0}2 ... {0}{1}", param_name, n)
                } else {
                    let mut s = String::new();
                    for i in 0..n {
                        s.push_str(param_name);
                        if i <= n - 1 {
                            s.push_str(" ");
                        }
                    }
                    s
                }
            },
            MultipleArguments::ZeroOrMore => {
                format!("[{0} [{0} ...]]", param_name)
            },
            MultipleArguments::OneOrMore => {
                format!("{0} [{0} [{0} ...]]", param_name)
            },
        }
    }
}

/// An argument with a single parameter.
#[derive(Debug, Clone)]
struct SingleArgument;

impl SingleArgument {
    /// Creates a description of the parameters for a usage string.
    fn usage(&self, param_name: &str) -> String {
        String::from(param_name)
    }
}

impl SingleArgument {
    /// Creates a description of the parameters for a usage string.
    fn usage(&self) -> String {
        String::new()
    }
}

// =============================================================================
// ========================= Specialized arguments =============================
// =============================================================================

/// A required single-parameter argument.
#[derive(Debug, Clone)]
pub struct RequiredSingleArgument<'a> {
    name: &'a str,
    desc: ArgumentDescription<'a>,
}

impl <'a> RequiredSingleArgument<'a> {
    pub fn add_to(&self, parser: &mut ArgumentParser<'a>) 
            -> Result<RequiredSingleTag, String> {
        let id = try!(parser.add_required_single(self));
        Ok(RequiredSingleTag { id: id })
    }
}

/// A required multiple-parameter argument.
#[derive(Debug, Clone)]
pub struct RequiredMultipleArguments<'a> {
    name: &'a str,
    argtype: MultipleArguments,
    help: Option<&'a str>,
}

impl <'a> RequiredMultipleArguments<'a> {
    pub fn add_to(&self, parser: &mut ArgumentParser<'a>) 
            -> Result<RequiredMultiplesTag, String> {
        let id = try!(parser.add_required_multiple(self));
        Ok(RequiredMultiplesTag { id: id })
    }
}

/// A flag that interrupts the parsing when encountered.
#[derive(Debug, Clone)]
pub struct InterruptFlag<'a> {
    name: OptionalName<'a>,
    desc: ArgumentDescription<'a>,
}

impl <'a> InterruptFlag<'a> {
    pub fn add_to(&self, parser: &mut ArgumentParser<'a>)
            -> Result<InterruptFlagTag, String> {
        parser.add_interrupt_flag(self)
    }
}

/// A flag that is set to true when the argument is found.
#[derive(Debug, Clone)]
pub struct FlagArgument<'a> {
    name: OptionalName<'a>,
    desc: ArgumentDescription<'a>,
}

impl <'a> FlagArgument<'a> {
    pub fn add_to(&self, parser: &mut ArgumentParser<'a>) 
            -> Result<FlagTag, String> {
        let id = try!(parser.add_flag(self));
        Ok(FlagTag { id: id })
    }
}

// =============================================================================
// ======================== Argument count builders ============================
// =============================================================================

/// A required argument.
#[derive(Debug)]
pub struct Required<'a> {
    name: &'a str,
}

impl <'a> Required<'a> {
    pub fn single(self) -> RequiredSingleArgument<'a> {
        RequiredSingleArgument {
            name: self.name,
            desc: ArgumentDescription::empty(),
        }
    }
}

#[derive(Debug, Clone)]
enum OptionalName<'a> {
    Short(char),
    Long(&'a str),
    ShortAndLong(char, &'a str),
}

impl <'a> OptionalName<'a> {
    fn create_names(&self) -> Vec<String> {
        match self {
            &OptionalName::Short(ch) => {
                vec![format!("-{}", ch)]
            },
            &OptionalName::Long(long) => {
                vec![format!("--{}", long)]
            },
            &OptionalName::ShortAndLong(ch, long) => {
                vec![format!("-{}", ch), format!("--{}", long)]
            }
        }
    }
}

/// An optional argument
#[derive(Debug)]
pub struct Optional<'a> {
    name: OptionalName<'a>,
}

impl <'a> Optional<'a> {
    /// Turns the argument into an interrupt flag, which interrupts the parse
    /// and returns the tag when encountered.
    pub fn interrupt(self) -> InterruptFlag<'a> {
        InterruptFlag {
            name: self.name,
            desc: ArgumentDescription::empty(),
        }
    }
    
    /// Turns the argument into a flag, which will be true if it is found in
    /// the arguments given to the program.
    pub fn flag(self) -> FlagArgument<'a> {
        FlagArgument {
            name: self.name,
            desc: ArgumentDescription::empty(),
        }
    }
}


// =============================================================================
// ================================== Tags =====================================
// =============================================================================

/// The handle for a required single-parameter argument.
#[derive(Debug, Clone, PartialEq)]
pub struct RequiredSingleTag {
    id: Id
}

impl RequiredSingleTag {
    /// Gets the value of this argument in the parsed arguments.
    pub fn get<'a>(&self, arguments: &ParsedArguments<'a>) -> &'a str {
        arguments.get_required_single(&self.id)
    }
}

/// The handle for a required multiple-parameter argument.
#[derive(Debug, Clone, PartialEq)]
pub struct RequiredMultiplesTag {
    id: Id
}

impl RequiredMultiplesTag {
    /// Gets the value of this argument in the parsed arguments.
    pub fn get<'a>(&self, arguments: &'a ParsedArguments<'a>) -> &Vec<&'a str> {
        arguments.get_required_multiple(&self.id)
    }
}

/// The handle for an interrupt flag argument.
#[derive(Debug, Clone, PartialEq)]
pub struct InterruptFlagTag {
    id: Id
}

/// The handle for a flag argument.
#[derive(Debug, Clone, PartialEq)]
pub struct FlagTag {
    id: Id
}

impl FlagTag {
    /// Gets the value of this argument in the parsed arguments.
    pub fn get<'a>(&self, arguments: &ParsedArguments<'a>) -> bool {
        arguments.get_flag(&self.id)
    }
}

// =============================================================================
// ========================== Argument constructors ============================
// =============================================================================

/// An argument for the parser.
pub struct Argument;

impl <'a> Argument {
    /// Creates a builder for a new required argument with the given name.
    pub fn required(name: &'a str) -> Required<'a> {
        Required {
            name: name
        }
    }
    
    /// Creates a builder for an optional argument with the given short name
    /// prefixed by '-' (eg '-a').
    pub fn optional_short(name: char) -> Optional<'a> {
        Optional {
            name: OptionalName::Short(name)
        }
    }
    
    /// Creates a builder for an optional argument with the given long name
    /// prefixed by '--' (eg '--all').
    pub fn optional_long(name: &'a str) -> Optional<'a> {
        Optional {
            name: OptionalName::Long(name)
        }
    }
    
    /// Creates a builder for an optional argument with the given short and
    /// long names, where the short name is prefixed by '-' and the long name by 
    /// '--'.
    pub fn optional_short_and_long(short_name: char, long_name: &'a str) 
            -> Optional<'a> {
        Optional {
            name: OptionalName::ShortAndLong(short_name, long_name)
        }
    }
}

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

impl <'a> ParsedArguments<'a> {
    fn get_required_single(&self, id: &Id) -> &'a str {
        self.req_singles.get(id).unwrap_or_else(|| panic!(format!(
            "No required single argument found with id {}", id
        )))
    }
    
    fn get_required_multiple(&'a self, id: &Id) -> &'a Vec<&'a str> {
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
    
    fn get_optional_single(&'a self, id: &Id) -> &'a Option<&'a str> {
        self.opt_singles.get(id).unwrap_or_else(|| panic!(format!(
            "No optional single-parameter argument found with id {}", id
        )))
    }
    
    fn get_optional_multiple(&'a self, id: &Id) -> &'a Option<Vec<&'a str>> {
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

// =============================================================================
// ================================= Parser ====================================
// =============================================================================

/// The result of a succesful parse. Either all the arguments are parsed and
/// bound, or an interrupt flag is encountered and the handle of the
/// corresponding argument is returned.
pub enum ParseStatus<'a> {
    Parsed(ParsedArguments<'a>),
    Interrupted(InterruptFlagTag),
}

/// The result of a parse attempt.
pub type ParseResult<'a> = Result<ParseStatus<'a>, String>;

/// Creates the description strings for an argument with the given 
/// names, parameter name and description.
fn create_description(names: &Vec<String>, argname: &str, description: &str) 
        -> (String, String) {
    ("".into(), "".into())
}


#[derive(Debug, Clone)]
pub struct ArgumentParser<'a> {
    pub title: &'a str,
    next_id: Id,
    req_singles: Vec<Id>,
    req_vararg: Option<(Id, MultipleArguments)>,
    opt_singles: HashMap<String, Id>,
    opt_multiples: HashMap<String, (Id, MultipleArguments)>,
    opt_flags: HashMap<String, Id>,
    interrupt_flags: HashMap<String, InterruptFlagTag>,
    taken_names: HashSet<String>,
    req_descriptions: Vec<(String, String)>,
    opt_descriptions: Vec<(String, String)>,
}

impl <'a> ArgumentParser<'a> {
    /// Creates a new argument parser with the given title.
    pub fn new(title: &str) -> ArgumentParser {
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
    fn check_names(&self, names: &Vec<String>) -> Result<(), String> {
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
    fn add_required_single(&mut self, arg: &RequiredSingleArgument<'a>) 
            -> Result<Id, String> {
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
            Ok(id)
        }
    }
    
    /// Attempts to add a required multiple-parameter argument.
    fn add_required_multiple(&mut self, arg: &RequiredMultipleArguments<'a>)
            -> Result<Id, String> {
        if self.req_vararg.is_some() {
            Err(String::from(
                "A required multi-count argument is already defined\
            "))
        } else {
            let id = self.generate_id();
            self.req_vararg = Some((id, arg.argtype.clone()));
            Ok(id)
        }
    }
    
    /// Attempts to add an interrupt flag.
    fn add_interrupt_flag(&mut self, flag: &InterruptFlag<'a>)
            -> Result<InterruptFlagTag, String> {
        let names = flag.name.create_names();
        try!(self.check_names(&names));
        let id = self.generate_id();
        let tag = InterruptFlagTag { id: id };
        for name in names {
            self.interrupt_flags.insert(name.clone(), tag.clone());
            self.taken_names.insert(name);
        }
        Ok(tag)
    }
    
    /// Attempts to add a flag.
    fn add_flag(&mut self, flag: &FlagArgument<'a>) 
            -> Result<Id, String> {
        let names = flag.name.create_names();
        try!(self.check_names(&names));
        let id = self.generate_id();
        for name in names {
            self.opt_flags.insert(name.clone(), id);
            self.taken_names.insert(name);
        }
        Ok(id)
    }
    
    /// Registers a default help command for the parser with the flags
    /// '-h' and '--help'.
    pub fn add_default_help_interrupt(&mut self) 
            -> Result<InterruptFlagTag, String> {
        let arg = Argument::optional_short_and_long('h', "help").interrupt();
        arg.add_to(self)
    }
    
    /// Registers a default version command for the parser with the flag
    /// '--version'.
    pub fn add_default_version_interrupt(&mut self) 
            -> Result<InterruptFlagTag, String> {
        let arg = Argument::optional_long("version").interrupt();
        arg.add_to(self)
    }
    
    /// Returns a result indicating whether the parser takes the given argument.
    pub fn check_argument(&self, argument: &String) -> Result<(), String> {
        if ! self.taken_names.contains(argument) {
            return Err(format!(
                "Unrecognized argument: '{}'", argument
            ))
        }
        Ok(())
    }
    
    /// Parses the given arguments or returns an error if they do not satisfy
    /// the declared arguments of the parser.
    pub fn parse(&self, args: &[String] )
            -> ParseResult<'a> {
       
        use self::ParseStatus::*;
        
        // Check for interrupts first
        for arg in args {
            if let Some(tag) = self.interrupt_flags.get(arg) {
                return Ok(Interrupted(tag.clone()))
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
        while i < args.len() {
            let ref arg = args[i];
            
            // Long flags
            if arg.starts_with("--") {
                i += 1; // TODO: HANDLE!
            // Short flags
            } else if arg.starts_with("-") {
                // Single short flag
                if arg.len() == 2 {
                    try!(self.check_argument(arg));
                    if let Some(id) = self.opt_flags.get(arg) {
                        opt_flags.insert(*id, true);
                    } else {
                        unimplemented!();
                    }
                    i += 1; // TODO: HANDLE!
                // Multiple short flags
                } else { 
                    for letter in arg.chars().skip(1) {
                        let name = format!("-{}", letter);
                        try!(self.check_argument(&name));
                        if let Some(id) = self.opt_flags.get(&name) {
                            opt_flags.insert(*id, true);
                        } else {
                            return Err(format!(
                                "The short argument '{}' is not a flag, and \
                                so cannot be grouped with other flags.\
                            ", name));
                        }
                    }
                    i += 1;
                }
            
            // Regular arguments
            } else {
                i += 1; // TODO: HANDLE
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