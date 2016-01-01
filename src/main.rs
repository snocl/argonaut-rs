#![allow(unused, dead_code)]

#[macro_use]
mod macros;

mod parser;

use parser::{ArgumentParser, Argument};
use parser::ParseStatus::{Parsed, Interrupted};
use std::env;

fn main() {
    println!("Argonaut!");
    let args: Vec<String> = env::args().skip(1).collect();
    let mut parser = ArgumentParser::new("Argonaut");
    
    let int_help = parser.add_default_help_interrupt().unwrap();
    let int_version = parser.add_default_version_interrupt().unwrap();
    //let one = Argument::required("one").single().add_to(&mut parser).unwrap();
    
    let two = Argument::optional_short_and_long('t', "two").flag()
        .add_to(&mut parser).unwrap();
    
    let f = Argument::optional_short('f').flag().add_to(&mut parser).unwrap();

    //println!("Parser: {:?}", parser);
    //println!("Tag: {:?}", one);

    match parser.parse(&args) {
        Err(err) => println!("Parse Error: {}", err),
        
        Ok(Parsed(parsed)) => {
            println!("Parser result:");
            println!("{:?}", parsed);
            println!("");
            //let res = one.get(&parsed);
            //println!("one: {}", res);
            println!("Two: {}", two.get(&parsed));
            println!("f: {}", f.get(&parsed));
        },
        
        Ok(Interrupted(tag)) => {
            if tag == int_help {
                println!("Help requested!");
            } else if tag == int_version {
                println!("0.1.0");
            } else {
                println!("Interrupt: {:?}", tag);
            }
        }
    }
}
