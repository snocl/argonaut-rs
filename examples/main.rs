extern crate argonaut;

use argonaut::{Parser, Arg, generate_help};
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

    // println!("Args: {:?}", args);

    let mut parser = Parser::new();

    // Create the arguments
    let a_foo = Arg::positional("foo").set_help("A single argument.");
    let a_foobar = Arg::required_trail("foobar").set_help("One or more trailing arguments.");
    let a_help = Arg::named_and_short("help", 'h')
                     .interrupt()
                     .set_help("Prints a help message for this tool and exits.");
    let a_version = Arg::named("version")
                        .interrupt()
                        .set_help("Prints the version of this tool and exits.");
    let a_verbose = Arg::named_and_short("verbose", 'v')
                        .switch()
                        .set_help("A switch (present or non-present)");
    let a_exclude = Arg::named_and_short("exclude", 'x')
                        .single()
                        .set_param("item")
                        .set_help("A single argument.");
    let a_extra = Arg::named_and_short("extra", 'e')
                      .zero_or_more()
                      .set_help("Zero or more arguments.");
    let a_add = Arg::named_and_short("add", 'a')
                    .one_or_more()
                    .set_param("number")
                    .set_help("One or more arguments.");
    let a_passed = Arg::named("")
                       .passalong()
                       .set_param("args")
                       .set_help("Collect the remaining arguments after this one.");

    // Add the arguments, and assert that none of the named ones overlap
    // Add one
    parser.define_single(a_foo).unwrap();
    // Add many
    parser.define(&[a_foobar, a_help, a_version, a_verbose, a_exclude, a_extra, a_add, a_passed])
          .unwrap();

    let mut foo = "";
    let mut foobar = Vec::new();
    let mut extra = None;
    let mut add = None;
    let mut verbose = false;
    let mut exclude = None;
    let mut passed = None;

    let usage = "Usage: cargo run --example main -- [--help | OPTIONS ]";

    for item in parser.parse(&args) {
        match item {
            Err(err) => {
                println!("Parse error: {:?}", err);
                println!("{}", usage);
                return;
            }
            Ok(Positional { name: "foo", value }) => {
                foo = value;
            }
            Ok(Trail { values }) => {
                foobar = values;
            }
            Ok(Interrupt { name: "help" }) => {
                return println!("{}\n\n{}", usage, generate_help(&parser));
            }
            Ok(Interrupt { name: "version" }) => {
                return println!("{}", env!("CARGO_PKG_VERSION"));
            }
            Ok(Switch { name: "verbose" }) => {
                verbose = true;
            }
            Ok(Single { name: "exclude", parameter }) => {
                exclude = Some(parameter);
            }
            Ok(Multiple { name: "add", parameters }) => {
                add = Some(parameters);
            }
            Ok(Multiple { name: "extra", parameters }) => {
                extra = Some(parameters);
            }
            Ok(PassAlong { name: "", args }) => {
                passed = Some(args);
            }
            _ => unreachable!(),
        }
    }
    // Use the parsed values
    println!("Parsed succesfully!");
    println!("Foo:          {}", foo);
    println!("Foobar:       {:?}", foobar);
    println!("Verbose:      {}", verbose);
    println!("Exclude:      {:?}", exclude);
    println!("Extra:        {:?}", extra);
    println!("Add:          {:?}", add);
    println!("Passed args:  {:?}", passed);
}
