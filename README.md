# dynarg

A simple dynamic argument system

### Example
```rust
/// Where normally you'd need to have a fixed set of arguments,
/// each of which would be roughly fixed types
/// -- you can dynamically push arguments on the fly instead.
/// This is useful when you need a consistent function signature
/// for different types of functions,
/// each needing different arguments.
fn draw(args: &mut dynarg::Args) {
    if let Some(arg) = args.get("thing_to_draw") {
        println!("I will draw {}!", arg);
    } else {
        panic!("Nothing to draw D:");
    }
}


fn main() {
    let mut args = dynarg::Args::defaults();

    // This is how you add arguments
    args.insert("thing_to_draw", dynarg::ArgData::String("apple".to_owned()));
    draw(&mut args);

    if !args.all_used() {
        println!("Warning! I didn't use all my arguments D:");
    }
    // Clear all the used flags on args
    args.reset_status();
}
```

## Todo

- [ ] Implement custom enum handling (allowing the user to implement their own `ArgData`)
- [ ] Add more example code