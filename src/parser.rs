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
    pub fn add_to(&self, parser: &mut ArgumentParser<'a>) -> RequiredSingleTag {
        let id = parser.add_required_single(self);
        RequiredSingleTag { id: id }
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
    Short(&'a str),
    Long(&'a str),
    ShortAndLong(&'a str, &'a str),
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
    pub fn get<'a>(&self, arguments: &ParsedArguments<'a>) -> &'a str {
        arguments.get_required_single(self.id)
    }
}


// =============================================================================
// ============================ Parsed arguments ===============================
// =============================================================================

#[derive(Debug)]
pub struct ParsedArguments<'a> {
    req_singles: HashMap<Id, &'a str>,
    //req_multiple: Vec<&'a str>,
    //opt_singles: HashMap<Id, Option<&'a str>>,
    //opt_multiples: HashMap<Id, Option<Vec<&'a str>>>,
    //opt_flags: HashMap<Id, bool>,
}

impl <'a> ParsedArguments<'a> {
    fn get_required_single(&self, id: Id) -> &'a str {
        "Yay!"
    }
}

// =============================================================================
// ========================== Argument constructors ============================
// =============================================================================

/// An argument for the parser
pub struct Argument;

impl <'a> Argument {
    pub fn required(name: &'a str) -> Required<'a> {
        Required {
            name: name
        }
    }
    
    pub fn optional_short(name: &'a str) -> Optional {
        Optional {
            name: OptionalName::Short(name)
        }
    }
    
    pub fn optional_long(name: &'a str) -> Optional<'a> {
        Optional {
            name: OptionalName::Long(name)
        }
    }
    
    pub fn optional_short_and_long(short_name: &'a str, long_name: &'a str) 
            -> Optional<'a> {
        Optional {
            name: OptionalName::ShortAndLong(short_name, long_name)
        }
    }
}

// =============================================================================
// ================================= Parser ====================================
// =============================================================================

#[derive(Debug)]
pub struct ArgumentParser<'a> {
    title: &'a str,
    next_id: Id,
    req_singles: Vec<(Id, RequiredSingleArgument<'a>)>,
    //req_multiple: Option<>,
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
            //req_multiple: None,
            //opt_multiples: Vec::new(),
            //opt_singles: Vec::new(),
            //opt_flags: Vec::new(),
        }
    }
    
    fn add_required_single(&mut self, arg: &RequiredSingleArgument<'a>) -> Id {
        let id = self.next_id;
        self.next_id += 1;
        self.req_singles.push((id, arg.clone()));
        id
    }
    
    pub fn parse(&self, args: &[&str] )
            -> Result<ParsedArguments<'a>, String> {
        let mut req_singles = HashMap::new();
        
        let mut parsed = ParsedArguments { 
            req_singles: req_singles
        };
        
        for arg in args.iter() {
            println!("Parsing '{}'", arg);
        }
        
        Ok(parsed)
    }
}