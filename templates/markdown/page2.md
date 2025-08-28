#ch01-02-hello-world

`println!`, the (!) marks it as macro, without its a function.
`rustc main.rs`, compiles code to binary.

#ch01-03-hello-cargo

Updating / Version Check
- `rustup update`, updates rust/components to the latest version
- `rustc --version`, prints the installed rust version to the console

`cargo new name`, creates a new project, name needs to be Camel-case.
`cargo check`, builds the project without prod. a binary, used for checking errors.
`cargo build`, builds the project, has extra args. for modes. ex. ``--release``, default: `debug`.
`cargo run`, builds and runs the project, has extra args. for modes. ex. `--release`. default: `debug`.
 - instead of saving the result of the build in the same dir., it gets stored in *./target/debug*.

Build Modes:
- `debug`, compiles without optimizations, stores in */target/debug*
	- PROS: made for development, rebuild quickly and often; CONS: code runs slow, binary size
- `--release`, compiles with optimizations, stores in *./target/release*
	- PROS: code runs faster; CONS: longer compile time

#ch02-00-guessing-game-tutorial

*"the program will generate a random integer between 1 and 100. It will then prompt the player to enter a guess. After a guess is entered, the program will indicate whether the guess is too low or too high. If the guess is correct, the game will print a congratulatory message and exit."*

std::io => standard input / output
io::stdin => io std console input

`prelude`, is the name of a set of items defined in std that it brings into the scope of every program.
`use`, if a type isn't in the *prelude*, you bring it into the scope with it.
`fn main() {}`, declares a new function called main, empty *()* = no parameters, *{}* = function body.
`let`, to create a variable, can only be used in a function (local scope).
- variables are *immutable* by default, means once the var. has a value, it wont change.
`String:new()`, returns a new instance of an empty *type String*, grow-able, UTF-8 encoded text.
`&`, indicates argument as *reference*, let multiple parts of code access on piece of data without needing to copy that data into memory multiple times.
`.expect()`, handling Potential failure with *Result*, like your parents, it expects you to fail.
`"{}"`, placeholder that holds values, insert variables or expressions in `{}` and use `.format()` or f-strings for substitution.
`cargo add PKG_NAME`, add/install package to the project


#ch03-01-variables-and-mutability

`mut`, keyword defines that the variable is re-assignable / mutable, cannot be used with const!
```rust
let x = 1
let mut y = 2

x = 3 // will throw an error, var. x is not mutable (value cannot be re-assigned)
y = 4 // will compile, var. y is mutable (value can be re-assigned)
```

`const`, declare a constant variable, can be declared in any scope (local/global), UPPERCASE and UNDERSCORE
```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

`{}`, inside a function creates a new local scope.

I have Problems understanding Shadowing.

#ch03-02-data-types

| Length  | Signed  | Unsigned |
| ------- | ------- | -------- |
| 8-bit   | `i8`    | `u8`     |
| 16-bit  | `i16`   | `u16`    |
| 32-bit  | `i32`   | `u32`    |
| 64-bit  | `i64`   | `u64`    |
| 128-bit | `i128`  | `u128`   |
| arch    | `isize` | `usize`  |




https://rust-book.cs.brown.edu/ch03-02-data-types.html