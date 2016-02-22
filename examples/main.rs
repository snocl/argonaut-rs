// Created by Jakob Lautrup Nysom @ 31-12-2015
extern crate argonaut;

use argonaut::{Parser, Arg};
use std::env;

fn main() {
    use argonaut::StructuredArgument::*;
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
    let bar     = Arg::required_trail();
    let help    = Arg::named_and_short("help", 'h').interrupt();
    let version = Arg::named("version").interrupt();
    let verbose = Arg::named_and_short("verbose", 'v').switch();
    let exclude = Arg::named_and_short("exclude", 'x').single();
    let extra   = Arg::named_and_short("extra", 'e').zero_or_more();
    let add     = Arg::named_and_short("add", 'a').one_or_more();
    let passed  = Arg::named("").passalong();
    
    // Add them, and assert that none of the named ones overlap
    parser.add(&foo).unwrap();
    parser.add(&bar).unwrap();
    parser.add(&help).unwrap();
    parser.add(&version).unwrap();
    parser.add(&verbose).unwrap();
    parser.add(&exclude).unwrap();
    parser.add(&extra).unwrap();
    parser.add(&add).unwrap();
    parser.add(&passed).unwrap();
        
    let mut foo = "";
    let mut bar = Vec::new();
    let mut extra = None;
    let mut add = None;
    let mut verbose = false;
    let mut exclude = None;
    let mut passed = None;
    
    let usage = "Usage: cargo run --example main -- [--help] [options]";
    let help = "\
Required arguments:    
foo             a single argument
bar [bar, ..]   one or more arguments

Interrupts:
--help | -h     show this help message
--version       show the version of this library

Optional arguments:
--verbose | -v              a switch (present or non-present)
--extra | -e [arg, ..]      zero or more arguments
--add | -a arg [arg, ..]    one or more arguments
--                          collect the remaining arguments\
";
    
    for item in parser.parse(&args) {
        match item {
            Err(err) => {
                println!("Parse error: {:?}", err);
                println!("{}", usage);
                return;
            },
            Ok(Positional { name: "foo", value }) => {
                foo = value;
            },
            Ok(Trail { values }) => {
                bar = values;
            },
            Ok(Interrupt { name: "help" }) => {
                println!("{}\n{}", usage, help);
            },
            Ok(Interrupt { name: "version" }) => {
                println!("{}", env!("CARGO_PKG_VERSION"));
            },
            Ok(Switch { name: "verbose" }) => {
                verbose = true;
            },
            Ok(Single { name: "exclude", parameter }) => {
                exclude = Some(parameter);
            },
            Ok(Multiple { name: "add", parameters }) => {
                add = Some(parameters);
            },
            Ok(Multiple { name: "extra", parameters }) => {
                extra = Some(parameters);
            },
            Ok(PassAlong { name: "", args }) => {
                passed = Some(args);
            },
            _ => unreachable!(),
        }
    }
    // Use the parsed values
    println!("Parsed succesfully!");
    println!("Foo:          {}", foo);
    println!("Bar:          {:?}", bar);
    println!("Verbose:      {}", verbose);
    println!("Exclude:      {:?}", exclude);
    println!("Extra:        {:?}", extra);
    println!("Add:          {:?}", add);
    println!("Passed args:  {:?}", passed);
}
