// Created by Jakob Lautrup Nysom @ 31-12-2015

use std::collections::{HashMap, HashSet};
use std::iter::Iterator;

type Id = usize;

// Global id to help uniquely identify parsers and avoid misuse of argument
// handles with the output of other parsers.
// TODO: Find a thread-safe alternative to this, or use the panicky way?
//static mut PARSER_ID: Id = 1;

/// The name of an argument.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum ArgumentName<'a> {
    Short(char),
    Long(&'a str),
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

// Declare and implement all the handles and their 'get' methods.
tag_structs! {
    SingleRequiredTag: get_single_required -> &'a str,
    MultipleRequiredTag: get_multiple_required -> &Vec<&'a str>,
    SingleOptionalTag: get_single_optional -> &Option<&'a str>,
    MultipleOptionalTag: get_multiple_optional -> &Option<Vec<&'a str>>,
    FlagTag: get_flag -> bool
}

// =============================================================================
// ========================= Specialized arguments =============================
// =============================================================================

/*argument_structs! {
    /// A required single-parameter argument.
    SingleRequiredArgument {
        name: &'a str
    }
    add_single_required -> SingleRequiredTag
}*/

/*
/// A required multiple-parameter argument.
#[derive(Debug, Clone)]
pub struct MultipleRequiredArguments<'a> {
    name: &'a str,
    argtype: MultipleArguments,
    help: Option<&'a str>,
}

impl <'a> MultipleRequiredArguments<'a> {
    pub fn add_to(&self, parser: &mut ArgumentParser<'a>) 
            -> Result<MultipleRequiredTag, String> {
        let id = try!(parser.add_multiple_required(self));
        Ok(MultipleRequiredTag { id: id })
    }
}

/// A flag that interrupts the parsing when encountered.
#[derive(Debug, Clone)]
pub struct InterruptFlagArgument<'a> {
    name: OptionalName<'a>,
    help: Option<&'a str>,
}

impl <'a> InterruptFlagArgument<'a> {
    pub fn add_to(&self, parser: &mut ArgumentParser<'a>)
            -> Result<InterruptFlagArgumentTag, String> {
        parser.add_interrupt_flag(self)
    }
}

/// A flag that is set to true when the argument is found.
#[derive(Debug, Clone)]
pub struct FlagArgument<'a> {
    name: OptionalName<'a>,
    help: Option<&'a str>,
}

impl <'a> FlagArgument<'a> {
    pub fn add_to(&self, parser: &mut ArgumentParser<'a>) 
            -> Result<FlagTag, String> {
        let id = try!(parser.add_flag(self));
        Ok(FlagTag { id: id })
    }
}
*/

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

/*
/// A required argument.
#[derive(Debug)]
pub struct Required<'a> {
    name: &'a str,
}

impl <'a> Required<'a> {
    pub fn single(self) -> SingleRequiredArgument<'a> {
        SingleRequiredArgument {
            name: self.name,
            help: None,
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
    pub fn interrupt(self) -> InterruptFlagArgument<'a> {
        InterruptFlagArgument {
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
*/

// =============================================================================
// ========================== Argument constructors ============================
// =============================================================================

/// An argument for the parser.
pub struct Argument;

impl <'a> Argument {
    /// Creates a builder for a new required argument with the given name.
    pub fn required(name: &'a str) -> Required<'a> {
        Required {
            name: name,
            help: None,
        }
    }
    
    /// Creates a builder for an optional argument with the given short name
    /// prefixed by '-' (eg '-a').
    pub fn optional_short(name: char) -> Optional<'a> {
        Optional {
            name: OptionalName::Short(name),
            help: None,
        }
    }
    
    /// Creates a builder for an optional argument with the given long name
    /// prefixed by '--' (eg '--all').
    pub fn optional_long(name: &'a str) -> Optional<'a> {
        Optional {
            name: OptionalName::Long(name),
            help: None,
        }
    }
    
    /// Creates a builder for an optional argument with the given short and
    /// long names, where the short name is prefixed by '-' and the long name by 
    /// '--'.
    pub fn optional_short_and_long(short_name: char, long_name: &'a str) 
            -> Optional<'a> {
        Optional {
            name: OptionalName::ShortAndLong(short_name, long_name),
            help: None,
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

// =============================================================================
// ================================= Parser ====================================
// =============================================================================

/// The result of a succesful parse. Either all the arguments are parsed and
/// bound, or an interrupt flag is encountered and the handle of the
/// corresponding argument is returned.
pub enum ParseStatus<'a> {
    Parsed(ParsedArguments<'a>),
    Interrupted(InterruptFlagArgumentTag),
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
    opt_singles: HashMap<ArgumentName<'a>, Id>,
    opt_multiples: HashMap<ArgumentName<'a>, (Id, MultipleArguments)>,
    opt_flags: HashMap<ArgumentName<'a>, Id>,
    interrupt_flags: HashMap<ArgumentName<'a>, InterruptFlagArgumentTag>,
    taken_names: HashSet<ArgumentName<'a>>,
    req_descriptions: Vec<FullDescription<'a>>,
    opt_descriptions: Vec<FullDescription<'a>>,
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
    fn add_single_required(&mut self, arg: &SingleRequiredArgument<'a>) 
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
    fn add_multiple_required(&mut self, arg: &MultipleRequiredArguments<'a>)
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
    fn add_interrupt_flag(&mut self, flag: &InterruptFlagArgument<'a>)
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
    fn add_flag(&mut self, flag: &FlagArgument<'a>) 
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
    
    /// Registers a default help command for the parser with the flags
    /// '-h' and '--help'.
    pub fn add_default_help_interrupt(&mut self) 
            -> Result<InterruptFlagArgumentTag, String> {
        let arg = Argument::optional_short_and_long('h', "help").interrupt();
        arg.add_to(self)
    }
    
    /// Registers a default version command for the parser with the flag
    /// '--version'.
    pub fn add_default_version_interrupt(&mut self) 
            -> Result<InterruptFlagArgumentTag, String> {
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