#![allow(unused, dead_code)]
mod parser;
use parser::{Parser, Arg};

fn main() {
    println!("Hello, world!");
    let mut parser = Parser::new("Argonaut");
    let arg = Arg::required("one").single();
    let one = arg.add_to(&mut parser);
    let parsed = parser.parse(&["hello", "world"]).unwrap();
    println!("Parser: {:?}", parser);
    println!("Arg: {:?}", arg);
    println!("Tag: {:?}", one);
    let res = one.get(&parsed);
    println!("one: {}", res);
}
