#![allow(unused, dead_code)]

#[macro_use]
mod macros;

mod parser;

use parser::{ArgumentParser, Required, Optional};
use parser::ParseStatus::{Parsed, Interrupted};
use std::env;

fn main() {
    println!("Argonaut!");
    let arg_vec: Vec<_> = env::args().skip(1).collect();
    let args: Vec<&str> = arg_vec.iter().map(|s| s.as_str()).collect();
    let mut parser = ArgumentParser::new("Argonaut");
    
    //let one = Argument::required("one").single().add_to(&mut parser).unwrap();
    
    let two_arg = Optional::short_and_long('t', "two").flag();
    let two = two_arg.add_to(&mut parser).unwrap();
    
    let f_arg = Optional::short('f').flag();
    let f = f_arg.add_to(&mut parser).unwrap();

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
            {
                println!("Interrupt: {:?}", tag);
            }
        }
    }
}
