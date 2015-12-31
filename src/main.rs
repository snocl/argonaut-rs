#![allow(unused, dead_code)]
mod parser;
use parser::{ArgumentParser, Argument, ParseStatus};

fn main() {
    println!("Hello, world!");
    let mut parser = ArgumentParser::new("Argonaut");
    let arg = Argument::required("one").single();
    let one = arg.add_to(&mut parser).unwrap();
    let status = parser.parse(&["hello", "world"]);
    println!("Parser: {:?}", parser);
    println!("Arg: {:?}", arg);
    println!("Tag: {:?}", one);
    match status {
        ParseStatus::Ok(parsed) => {
            let res = one.get(&parsed);
            println!("one: {}", res);
        },
        ParseStatus::Interrupt(tag) => {
        
        },
        ParseStatus::Err(error) => { 
            println!("Parse error! ({})", error);
        }
    }
}
