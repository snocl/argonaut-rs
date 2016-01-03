// Created by Jakob Lautrup Nysom @ 31-12-2015
#![allow(unused, dead_code)]
#![feature(convert)]

mod parser;

use parser::{Parser, arg};
use parser::ParseStatus::{Parsed, Interrupted};
use std::env;


fn main() {
    println!("Argonaut!");
    let arg_vec: Vec<_> = env::args().skip(1).collect();
    let args: Vec<&str> = arg_vec.iter().map(|s| s.as_str()).collect();
    
    println!("Args: {:?}", args);
    
    let mut parser = Parser::new("Argonaut");
    
    let help = arg::opt_short_and_long('h', "help").interrupt_flag();
    parser.add(&help).unwrap();
    
    //println!("Parser: {:?}", parser);
    
    match parser.parse(&args) {
        Ok(Parsed(parsed)) => {
            println!("Parsed succesfully!");
        },
        Ok(Interrupted(flag)) => {
            println!("Interrupt flag!");
        }
        Err(reason) => {
            println!("Parse error: {}", reason);
        },
    }
    
    
    
}
