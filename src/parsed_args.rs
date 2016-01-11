// Created by Jakob Lautrup Nysom @ 05-01-2016

use std::collections::{HashMap};
use common::{FlagName, OptName, convert_flag_name};

/// The parsed arguments of a succesful parsing.
#[derive(Debug)]
pub struct ParsedArgs<'a> {
    positional: HashMap<&'a str, &'a str>,
    trail: Option<Vec<&'a str>>,
    singles: HashMap<OptName<'a>, Option<&'a str>>,
    multiples: HashMap<OptName<'a>, Option<Vec<&'a str>>>,
    switches: HashMap<OptName<'a>, bool>,
    aliases: HashMap<FlagName<'a>, OptName<'a>>,
}

impl<'a> ParsedArgs<'a> {
    
    /// Creates a new set of parsed arguments.
    pub fn new(positional: HashMap<&'a str, &'a str>,
            trail: Option<Vec<&'a str>>,
            singles: HashMap<OptName<'a>, Option<&'a str>>,
            multiples: HashMap<OptName<'a>, Option<Vec<&'a str>>>,
            switches: HashMap<OptName<'a>, bool>,
            aliases: HashMap<FlagName<'a>, OptName<'a>>) 
            -> ParsedArgs<'a> {
        ParsedArgs {
            positional: positional,
            trail: trail,
            singles: singles,
            multiples: multiples,
            switches: switches,
            aliases: aliases,
        }
    }
    
    /// Returns the positional argument with the given name if it exists.
    pub fn positional(&'a self, name: &'a str) -> Result<&'a str, String> {
        if let Some(arg) = self.positional.get(name) {
            Ok(arg)
        } else {
            Err(format!(
                "No positional argument named '{}' declared", name
            ))
        }
    }
    
    /// Accesses a flag with the given long name in the parsed arguments.
    pub fn named(&'a self, name: &'a str) -> ParsedArgsAccess<'a> {
        let opt_name = convert_flag_name(&self.aliases, &FlagName::Long(name));
        ParsedArgsAccess { name: opt_name, args: &self }
    }
    
    /// Returns the trail for the parsed arguments if any was specified.
    pub fn trail(&'a self) -> Option<Vec<&'a str>> {
        self.trail.clone()
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