// Created by Jakob Lautrup Nysom @ 31-12-2015

use std::collections::HashMap;
use std::iter::Iterator;

type Id = usize;

#[derive(Debug, Clone)]
struct ArgumentDescription<'a> {
    name: Option<&'a str>,
    help: Option<&'a str>,
}

impl <'a> ArgumentDescription<'a> {
    pub fn empty() -> ArgumentDescription<'a> {
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

// =============================================================================
// ======================== Argument count builders ============================
// =============================================================================

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

#[derive(Debug)]
pub struct Optional<'a> {
    name: OptionalName<'a>,
}


// =============================================================================
// ================================== Tags =====================================
// =============================================================================

#[derive(Debug, Clone)]
pub struct RequiredSingleTag {
    id: Id
}

impl RequiredSingleTag {
    /// Gets the value of this tag in the parsed arguments
    pub fn get<'a>(&self, arguments: &ParsedArguments<'a>) -> &'a str {
        arguments.get_required_single(&self.id)
    }
}

#[derive(Debug, Clone)]
pub struct RequiredMultiplesTag {
    id: Id
}

impl RequiredMultiplesTag {
    /// Gets the value of this tag in the parsed arguments
    pub fn get<'a>(&self, arguments: &'a ParsedArguments<'a>) -> &Vec<&'a str> {
        arguments.get_required_multiple(&self.id)
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

#[derive(Debug, Clone)]
pub struct ArgumentParser<'a> {
    pub title: &'a str,
    next_id: Id,
    req_singles: Vec<(Id, RequiredSingleArgument<'a>)>,
    req_vararg: Option<(Id, RequiredMultipleArguments<'a>)>,
    //opt_multiples: Vec<(Id, <'a>)>,
    //opt_singles: Vec<(Id, <'a>)>,
    //opt_flags: Vec<(Id, <'a>)>,
}

impl <'a> ArgumentParser<'a> {
    pub fn new(title: &str) -> ArgumentParser {
        ArgumentParser {
            title: title,
            next_id: 1,
            req_singles: Vec::new(),
            req_vararg: None,
            //opt_multiples: Vec::new(),
            //opt_singles: Vec::new(),
            //opt_flags: Vec::new(),
        }
    }
    
    fn generate_id(&mut self) -> Id {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
    
    fn add_required_single(&mut self, arg: &RequiredSingleArgument<'a>) 
            -> Result<Id, String> {
        if self.req_vararg.is_some() {
            Err(format!("Could not add the argument '{}', since all required 
                single-parameter arguments must be added before the 
                variable-parameter argument", arg.name))
        } else {
            let id = self.generate_id();
            self.req_singles.push((id, arg.clone()));
            Ok(id)
        }
    }
    
    fn add_required_multiple(&mut self, arg: &RequiredMultipleArguments<'a>)
            -> Result<Id, String> {
        if self.req_vararg.is_some() {
            Err(String::from("There is already a multi-count argument defined 
                for the parser"))
        } else {
            let id = self.generate_id();
            self.req_vararg = Some((id, arg.clone()));
            Ok(id)
        }
    }
    
    pub fn parse(&self, args: &[&str] )
            -> Result<ParsedArguments<'a>, String> {
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
        
        Ok(parsed)
    }
}