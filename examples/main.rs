// Created by Jakob Lautrup Nysom @ 31-12-2015
extern crate argonaut;

use argonaut::{Parser, Arg};
use argonaut::ParseStatus::{Parsed, Interrupted};
use std::env;


fn main() {
    println!("Argonaut!");
    let arg_vec: Vec<_> = env::args().skip(1).collect();
    let args: Vec<&str> = arg_vec.iter().map(|s| s.as_str()).collect();
    
    println!("Args: {:?}", args);
    
    let mut parser = Parser::new();
    
    let foo = Arg::positional("foo");
    let bar = Arg::required_trail("bar");
    let help = Arg::short_and_long('h', "help").interrupt_flag();
    let version = Arg::long("version").interrupt_flag();
    let verbose = Arg::short_and_long('v', "verbose").flag();
    let exclude = Arg::short_and_long('x', "exclude").single(Some("foo"));
    let extra = Arg::short('e').zero_or_more(None);
    let add = Arg::short_and_long('a', "add").one_or_more(None);
    
    parser.add(&foo).unwrap();
    parser.add(&bar).unwrap();
    parser.add(&help).unwrap();
    parser.add(&version).unwrap();
    parser.add(&verbose).unwrap();
    parser.add(&exclude).unwrap();
    parser.add(&extra).unwrap();
    parser.add(&add).unwrap();
    
    //println!("Parser: {:?}", parser);
    
    match parser.parse(&args) {
        Ok(Parsed(parsed)) => {
            println!("Parsed succesfully!");
            //println!("Result: {:?}", parsed);
            println!("Foo: {}",         parsed.positional(0).unwrap());
            println!("Bar: {:?}",       parsed.trail(1).unwrap());
            println!("Verbose: {}",     parsed.long("verbose").switch().unwrap());
            println!("Exclude: {:?}",   parsed.short('x').single().unwrap());
            println!("Extra: {:?}",     parsed.short('e').multiple().unwrap());
            println!("Add: {:?}",       parsed.long("add").multiple().unwrap());
        },
        Ok(Interrupted(flag)) => {
            println!("Interrupt flag!");
            if flag.is_short('h') {
                println!("Help requested!");
            } else if flag.is_long("version") {
                println!("Version ZERO POINT ZERO!");
            }
        }
        Err(reason) => {
            println!("Parse error: {:?}", reason);
        },
    }
    
    
    
}
