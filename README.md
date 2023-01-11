# dynarg

A simple dynamic argument system

### Example
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

- [ ] Replace `Option`s with `Result`s such that it's possible to identify whether the argument name didn't exist, or the type was wrong
- [ ] Add `snafu`
- [ ] Add convenience functions (e.g. `get_string()`, `get_int()`)
- [ ] Properly document gotchas