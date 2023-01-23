# dynarg


[![Build status](https://github.com/alxpettit/dynarg/workflows/CI/badge.svg)](https://github.com/alxpettit/dynarg/actions?query=workflow%3ACI)
[![crates.io](https://img.shields.io/crates/v/dynarg.svg)](https://crates.io/crates/dynarg)
[![Documentation](https://docs.rs/dynarg/badge.svg)](https://docs.rs/dynarg)

A simple dynamic argument system

Have you ever wanted to have multiple functions with the same signature,
but with very different purposes and behavior?

- Maybe you want to make an image editing app, with lots of tools.
- Maybe you want to make a Rust GCODE implementation.
- Maybe you just want to make a modular shell program and dynamically pick arguments out like fruits off a berry bush, 
while keeping track of which ones haven't been used.

Regardless, you probably want an API that can:

- Match arguments to static strings, to avoid runtime overhead,
while maintaining readability.
- Potentially handle dynamic strings if needed.
- Handle arbitrary argument types.
- Provide convenience functions for working with arguments 
-- e.g., wrapper functions for common types.

If any of this applies to you, this is a library to consider.
Note that for _very_ high-performance applications, 
it might be better to roll your own custom use case with a Vec (ideally recycled!) of enum, to avoid dynamic dispatch.

This API is at this point considered stable and reasonably mature. It is `forbid_unsafe`, and written in pure Rust.
### Basic example
```rust
use dynarg::Args;

fn main() {
    // Creating Args object
    let mut args = Args::new();
    // Inserting a string type
    args.insert_string("greeting", String::from("hello world"));
    // Inserting an i32 type
    args.insert_i32("meaning_of_life", 42);
    // There's a lot more types where that came from, BTW :)
    // (In fact, you can use any type that implements `Any`,
    // which... I think should be any?)
    
    // Retrieving string type
    let out = args.get_string("greeting").unwrap();
    println!("{}", out);
    // Retrieving i32
    let meaning_of_life = args.get_i32("meaning_of_life").unwrap();
    println!("The meaning of life is: {}", meaning_of_life);
}
```

### Tracking which argument is used


```rust
use dynarg::Args;

fn main() {
    // Creating Args object
    let mut args = Args::new();
    // Inserting a string type
    args.insert_string("greeting", String::from("hello world"));
    // Inserting an i32 type
    args.insert_i32("meaning_of_life", 42);
    // There's a lot more types where that came from, BTW :)
    // (In fact, you can use any type that implements `Any`, which... I think should be any?)
        
    // Retrieving string type (while marking used flag!)
    let out = args.poke_string("greeting").unwrap();
    println!("{}", out);
    // Retrieving i32 (also while marking used flag!)
    let meaning_of_life = args.poke_i32("meaning_of_life").unwrap();
    println!("The meaning of life is: {}", meaning_of_life);
    // NOTE: the difference between poke and poke_* functions, and get and get_,
    // is that poke marks the status as used.
    // Note that this only exists if `used` feature is enabled for library.
    // Explicitly marking status is used is useful for sanity checking -- e.g.
    
    // Checking used status of args is useful for catching
    // what would otherwise be silent or hard-to-catch errors
    if args.all_used() {
        println!("All used! :3");
    } else {
        for used_arg_name in args.iter_not_used_name() {
            println!("Arg: \"{}\" not used", used_arg_name);
        }
    }
}
```

### Less basic example

```rust
use dynarg::*;

/// Where normally you'd need to have a fixed set of arguments,
/// each of which would be roughly fixed types
/// -- you can dynamically push arguments on the fly instead.
/// This is useful when you need a consistent function signature
/// for different types of functions,
/// each needing different arguments
fn draw(args: &mut Args) {
    if let Ok(greeting) = args.poke_string("greeting") {
        println!("{} world", greeting);
    }
    if let Ok(arg) = args.poke::<Fruit>("fruit_to_draw") {
        println!("I will draw {}!", arg.0);
        if let Ok(size) = args.poke::<f32>("size") {
            println!("with a size of {}", size);
        }
    } else {
        panic!("Nothing to draw D:");
    }
}

/// A custom struct as an example
struct Fruit<'a>(&'a str);

fn main() {
   let mut args = Args::default();

    let apple = Fruit("apple");
    // This is how you add arguments
    args.insert("fruit_to_draw", Box::new(apple));
    args.insert("size", Box::new(5.2f32));
    let greeting = String::from("henlo");
    args.insert_string("greeting", greeting);
    draw(&mut args);
    if !args.all_used() {
        println!("Warning! I didn't use all my arguments D:");
    }
    // Clear all the used flags on args
    args.reset_used_status();
}
```

[Github available here.](https://github.com/alxpettit/dynarg)

PRs welcome :)

## Todo

- [x] Custom type handling
- [x] Replace `Option`s with `Result`s such that it's possible to identify whether the argument name didn't exist, or the type was wrong
- [x] Add `snafu`
- [x] Add convenience functions (e.g. `get_string()`, `get_int()`)
- [x] Properly document gotchas
- [x] Add variant without `used()` functionality.
- [x] Add more examples
- [ ] Benchmarks