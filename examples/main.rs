// Created by Jakob Lautrup Nysom @ 31-12-2015
extern crate argonaut;

use argonaut::{Parser, Arg};
use argonaut::ParseStatus::{Parsed, Interrupted};
use std::env;


fn main() {
    println!("Argonaut!");
    
    // Prepare the argument slice (skip the program path)
    let arg_vec: Vec<_> = env::args().skip(1).collect();
    let mut args: Vec<&str> = Vec::new();
    for arg in arg_vec.iter() {
        args.push(arg);
    }
    
    println!("Args: {:?}", args);
    
    let mut parser = Parser::new();
    
    // Create the arguments
    let foo     = Arg::positional("foo");
    let bar     = Arg::required_trail("bar");
    let help    = Arg::short_and_long('h', "help").interrupt_flag();
    let version = Arg::long("version").interrupt_flag();
    let verbose = Arg::short_and_long('v', "verbose").flag();
    let exclude = Arg::short_and_long('x', "exclude").single(Some("foo"));
    let extra   = Arg::short('e').zero_or_more(None);
    let add     = Arg::short_and_long('a', "add").one_or_more(None);
    
    // Add them, and assert that none of the named ones overlap
    parser.add(&foo).unwrap();
    parser.add(&bar).unwrap();
    parser.add(&help).unwrap();
    parser.add(&version).unwrap();
    parser.add(&verbose).unwrap();
    parser.add(&exclude).unwrap();
    parser.add(&extra).unwrap();
    parser.add(&add).unwrap();
    
    // Check the result
    match parser.parse(&args) {
        // The parse succeeded and all arguments were assigned
        Ok(Parsed(parsed)) => {
            println!("Parsed succesfully!");
            
            // get positional argument 0 and assert that a such exists
            println!("Foo: {}", parsed.positional(0).unwrap());
            
            // Get the trail from 1 and out
            println!("Bar: {:?}", parsed.trail(1).unwrap());
            
            // Check a 'switch' by its long name
            println!("Verbose: {}", parsed.long("verbose").switch().unwrap());
            
            // Check flag taking a single parameter by its short name
            println!("Exclude: {:?}", parsed.short('x').single().unwrap());
            
            // Check a flag taking multiple parameters by its short name
            println!("Extra: {:?}", parsed.short('e').multiple().unwrap());
            
            // Check a flag taking multiple parameters by its long name
            println!("Add: {:?}", parsed.long("add").multiple().unwrap());
        },
        
        // The parse succeeded, by finding one of the 'interrupt flags'
        Ok(Interrupted(flag)) => {
            println!("Interrupt flag!");
            
            // Test whether it is the given short argument
            if flag.is_short('h') {
                println!("Help requested!");
            
            // Test whether it is the given long argument
            } else if flag.is_long("version") {
                println!("Version ZERO POINT ZERO!");
            }
        }
        
        // The parse failed, due to the given error
        Err(error) => {
            println!("Parse error: {:?}", error);
        },
    }
    
    
    
}
