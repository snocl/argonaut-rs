# Argonaut
An argument parser for Rust, that grants as much control over the parser as possible.

# Example
This can be found in *examples/main.rs* as well, and be run with ```cargo run --example main```.

```rust
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
    let bar     = Arg::required_trail();
    let help    = Arg::named_and_short("help", 'h').interrupt();
    let version = Arg::named("version").interrupt();
    let verbose = Arg::named_and_short("verbose", 'v').switch();
    let exclude = Arg::named_and_short("exclude", 'x').single();
    let extra   = Arg::named_and_short("extra", 'e').zero_or_more();
    let add     = Arg::named_and_short("add", 'a').one_or_more();
    
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
            
            // Get positional argument 'foo'
            println!("Foo: {}", parsed.positional("foo").unwrap());
            
            // Get the trail (remaining arguments after the declared ones)
            println!("Bar: {:?}", parsed.trail().unwrap());
            
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
            } else if flag.is("version") {
                println!("Version ZERO POINT ZERO!");
            }
        }
        
        // The parse failed, due to the given error
        Err(error) => {
            println!("Parse error: {:?}", error);
        },
    } 
}

```

# Terminology

Flag: The identifying token for an argument (--flag)
Switch: A flag with no arguments that is either *there* or *not there*
Parameter: Tokens following an argument.
Trail: Zero or more tokens that follow the required positional arguments.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.