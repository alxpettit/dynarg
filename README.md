# dynarg

A simple dynamic argument system

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
    // (In fact, you can use any type that implements `Any`, which... I think should be any?)
    
    // Retrieving string type
    let out = args.get_string("greeting").unwrap();
    println!("{}", out);
    // Retrieving i32
    let meaning_of_life = args.get_i32("meaning_of_life").unwrap();
    println!("The meaning of life is: {}", meaning_of_life);
}
```

### Less basic example

```rust
use dynarg::{ArgData, Args};

/// Where normally you'd need to have a fixed set of arguments, each of which would be roughly fixed types
/// -- you can dynamically push arguments on the fly instead.
/// This is useful when you need a consistent function signature for different types of functions,
/// each needing different arguments
fn draw(args: &mut Args) {
    if let Some(arg) = args.get::<Fruit>("fruit_to_draw") {
        println!("I will draw {}!", arg.0);
        if let Some(size) = args.get::<f32>("size") {
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

    draw(&mut args);
    if !args.all_used() {
        println!("Warning! I didn't use all my arguments D:");
    }
    // Clear all the used flags on args
    args.reset_status();
}
```

## Todo

- [x] Custom type handling
- [x] Replace `Option`s with `Result`s such that it's possible to identify whether the argument name didn't exist, or the type was wrong
- [x] Add `snafu`
- [x] Add convenience functions (e.g. `get_string()`, `get_int()`)
- [x] Properly document gotchas
- [x] Add variant without `used()` functionality.
- [x] Add more examples