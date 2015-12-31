# Approaches
## Macros
### Advantages
Less writing required
Might be simpler to use (since it's possible to generate a nice struct for the parsed arguments)
### Drawbacks
Less sane errors
Less help from the compiler in getting syntax and usage right
Requires more documentation to make sense

## Object
### Advantages
Like normal Rust code
Support from the compiler
IDE support possible

### Drawbacks
Harder to convert from arguments to values
More verbose

# How do I polymorph
## Macros
Create the types with a macro and use fromstr and alike.

## Into (somehow)
Try to convert what I've got into whatever the user requests

## FromStr 
Do like rust-argparse and get the places to put the options in, then take advantage of knowing that type when filling in the arguments.