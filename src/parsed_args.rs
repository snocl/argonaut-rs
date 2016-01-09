// Created by Jakob Lautrup Nysom @ 05-01-2016

use std::collections::{HashMap};
use common::{FlagName, OptName, convert_flag_name};

/// The parsed arguments of a succesful parsing.
#[derive(Debug)]
pub struct ParsedArgs<'a> {
    positional: Vec<&'a str>,
    singles: HashMap<OptName<'a>, Option<&'a str>>,
    multiples: HashMap<OptName<'a>, Option<Vec<&'a str>>>,
    switches: HashMap<OptName<'a>, bool>,
    aliases: HashMap<FlagName<'a>, OptName<'a>>,
}

impl<'a> ParsedArgs<'a> {
    
    /// Creates a new set of parsed arguments.
    pub fn new(positional: Vec<&'a str>, 
            singles: HashMap<OptName<'a>, Option<&'a str>>,
            multiples: HashMap<OptName<'a>, Option<Vec<&'a str>>>,
            switches: HashMap<OptName<'a>, bool>,
            aliases: HashMap<FlagName<'a>, OptName<'a>>) 
            -> ParsedArgs<'a> {
        ParsedArgs {
            positional: positional,
            singles: singles,
            multiples: multiples,
            switches: switches,
            aliases: aliases,
        }
    }
    
    /// Accesses a flag with the given long name in the parsed arguments.
    pub fn long(&'a self, name: &'a str) -> ParsedArgsAccess<'a> {
        let opt_name = convert_flag_name(&self.aliases, &FlagName::Long(name));
        ParsedArgsAccess { name: opt_name, args: &self }
    }
    
    /// Accesses a flag with the given short name in the parsed arguments.
    pub fn short(&'a self, name: char) -> ParsedArgsAccess<'a> {
        let opt_name = convert_flag_name(&self.aliases, &FlagName::Short(name));
        ParsedArgsAccess { name: opt_name, args: &self }
    }
    
    /// Returns the value of given positional argument.
    pub fn positional(&self, index: usize) -> Result<&'a str, String> {
        if index >= self.positional.len() {
            Err(format!(
                "There are only {} (<= {}) positional arguments",
                self.positional.len(), index
            ))
        } else {
            Ok(self.positional[index])
        }
    }
    
    /// Returns the arguments after the given position.
    pub fn trail(&self, from: usize) -> Result<&[&'a str], String> {
        if from > self.positional.len() {
            Err(format!(
                "There are only {} (< {}), positional arguments", 
                self.positional.len(), from
            ))
        } else {
            Ok(&self.positional[from..])
        }
    }
}

/// An object to access a named member of the parsed arguments.
pub struct ParsedArgsAccess<'a> {
    name: OptName<'a>,
    args: &'a ParsedArgs<'a>,
}

impl<'a> ParsedArgsAccess<'a> {
    /// Returns the value of a switch.
    pub fn switch(self) -> Result<bool, String> {
        self.args.switches.get(&self.name).map(|b| *b).ok_or(
            format!("No switch found with the name '{}'", self.name)
        )
    }
    
    /// Returns the value of a flag with a single parameter.
    pub fn single(self) -> Result<Option<&'a str>, String> {
        self.args.singles.get(&self.name).map(|s| s.clone()).ok_or(
            format!("No single-parameter argument found with the name '{}'",
                self.name)
        )
    }
    
    /// Returns the value of a flag with multiple parameters.
    pub fn multiple(self) -> Result<Option<Vec<&'a str>>, String> {
        self.args.multiples.get(&self.name).map(|s| s.clone()).ok_or(
            format!("No multi-parameter argument found with the name '{}'",
                self.name)
        )
    }
}