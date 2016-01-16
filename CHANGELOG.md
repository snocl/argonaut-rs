# Changes
## 0.7.0
- ```Parser::parse``` now takes an ```&[&str]``` instead.

## 0.6.0
- The interrupt invariant of ```ParseStatus``` is now a ```&str```.
- The type ```OptName``` is no longer exported.

## 0.5.0
- Named arguments are now accessed with ```named``` on the parsed args.
- Parsed arguments can no longer be accessed with a short name.

## 0.4.0
- Named arguments are now required to have a 'long' name (eg --help).
- Renamed the optional argument constructors and the order of their arguments. 
- ```long``` becomes ```named```.
- ```short and long``` becomes ```named_and_short```.
- ```short``` is removed.
- Renamed the member function to check the value of an interrupt flag.
- ```is_long``` becomes ```is```.

## 0.3.1
- Updated cargo.toml to **also** say that the license is *either* **MIT** or **Apache 2.0**.

## 0.3.0
- Removed the ```.help(&str)``` member function from arguments, as it isn't useful in the current model.
- Removed the ```.param``` and ```.help``` member values from the arg struct.
- Made positional arguments be referred to by name as well.

## 0.2.0
- Changed the license to MIT or Apache-2.