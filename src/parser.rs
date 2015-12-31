// Created by Jakob Lautrup Nysom @ 31-12-2015

use std::collections::{HashMap, HashSet};
use std::iter::Iterator;

type Id = usize;

#[derive(Debug, Clone)]
struct ArgumentDescription<'a> {
    name: Option<&'a str>,
    help: Option<&'a str>,
}

impl <'a> ArgumentDescription<'a> {
    fn empty() -> ArgumentDescription<'a> {
        ArgumentDescription {
            name: None,
            help: None,
        }
    }
}

#[derive(Debug, Clone)]
enum MultipleArguments {
    Count(usize),
    ZeroOrMore,
    OneOrMore,
}

#[derive(Debug, Clone)]
struct SingleArgument;

#[derive(Debug, Clone)]
struct FlagArgument;

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
        let res = parser.add_required_single(self);
        res.map(|id| RequiredSingleTag { id: id })
    }
}

/// A required multiple-parameter argument.
#[derive(Debug, Clone)]
pub struct RequiredMultipleArguments<'a> {
    name: &'a str,
    argtype: MultipleArguments,
    desc: ArgumentDescription<'a>,
}

impl <'a> RequiredMultipleArguments<'a> {
    pub fn add_to(&self, parser: &mut ArgumentParser<'a>) 
            -> Result<RequiredMultiplesTag, String> {
        let res = parser.add_required_multiple(self);
        res.map(|id| RequiredMultiplesTag { id: id })
    }
}

/// A flag that interrupts the parsing when encountered.
#[derive(Debug, Clone)]
pub struct InterruptFlag<'a> {
    name: OptionalName<'a>,
    desc: ArgumentDescription<'a>,
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
    pub fn interrupt(self) -> InterruptFlag<'a> {
        InterruptFlag {
            name: self.name,
            desc: ArgumentDescription::empty(),
        }
    }
}


// =============================================================================
// ================================== Tags =====================================
// =============================================================================

/// The handle for a required single-parameter argument.
#[derive(Debug, Clone)]
pub struct RequiredSingleTag {
    id: Id
}

impl RequiredSingleTag {
    /// Gets the value of this tag in the parsed arguments.
    pub fn get<'a>(&self, arguments: &ParsedArguments<'a>) -> &'a str {
        arguments.get_required_single(&self.id)
    }
}

/// The handle for a required multiple-parameter argument.
#[derive(Debug, Clone)]
pub struct RequiredMultiplesTag {
    id: Id
}

impl RequiredMultiplesTag {
    /// Gets the value of this tag in the parsed arguments.
    pub fn get<'a>(&self, arguments: &'a ParsedArguments<'a>) -> &Vec<&'a str> {
        arguments.get_required_multiple(&self.id)
    }
}

/// The handle for an interrupt flag.
#[derive(Debug, Clone)]
pub struct InterruptFlagTag(Id);


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
                panic!(format!("The given tag has the wrong id for the
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
// ================================= Parser ====================================
// =============================================================================

/// The result of a succesful parse. Either all the arguments are parsed and
/// bound, or an interrupt flag is encountered and the handle of the
/// corresponding argument is returned.
#[must_use]
pub enum ParseStatus<'a> {
    Ok(ParsedArguments<'a>),
    Interrupt(InterruptFlagTag),
    Err(String),
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
    
    fn create_description(names: &Vec<String>, description: &str) 
            -> (String, String) {
        ("".into(), "".into())
    }
    
    /// Attempts to add a required single-parameter argument.
    fn add_required_single(&mut self, arg: &RequiredSingleArgument<'a>) 
            -> Result<Id, String> {
        if self.req_vararg.is_some() {
            Err(format!("Could not add the argument '{}', since all required 
                single-parameter arguments must be added before the 
                variable-parameter argument", arg.name))
        } else {
            let id = self.generate_id();
            self.req_singles.push(id);
            Ok(id)
        }
    }
    
    /// Attempts to add a required multiple-parameter argument.
    fn add_required_multiple(&mut self, arg: &RequiredMultipleArguments<'a>)
            -> Result<Id, String> {
        if self.req_vararg.is_some() {
            Err(String::from("There is already a multi-count argument defined 
                for the parser"))
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
        let tag = InterruptFlagTag(id);
        for name in names {
            self.interrupt_flags.insert(name, tag.clone());
        }
        Ok(tag)
    }
    
    /// Registers a default help command for the parser with the flags
    /// '-h' and '--help'.
    pub fn add_default_help_command(&mut self) {
        let arg = Argument::optional_short_and_long('h', "help");
    }
    
    /// Registers a default version command for the parser with the flag
    /// '--version'.
    pub fn add_default_version_command(&mut self, version: &'a str) {
        let arg = Argument::optional_long("version");
    }
    
    /// Parses the given arguments or returns an error if they do not satisfy
    /// the declared arguments of the parser.
    pub fn parse(&self, args: &[String] )
            -> ParseStatus<'a> {
                
        for arg in args {
            if let Some(tag) = self.interrupt_flags.get(arg) {
                return ParseStatus::Interrupt(tag.clone())
            }
        }
        
        
        let mut req_singles = HashMap::new();
        let mut req_vararg = None;
        let mut opt_singles = HashMap::new();
        let mut opt_multiples = HashMap::new();
        let mut opt_flags = HashMap::new();
        
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
        
        ParseStatus::Ok(parsed)
    }
}