use dynarg::*;

/// Where normally you'd need to have a fixed set of arguments, each of which would be roughly fixed types
/// -- you can dynamically push arguments on the fly instead.
/// This is useful when you need a consistent function signature for different types of functions,
/// each needing different arguments
fn draw(args: &mut Args) {
    if let Ok(arg) = args.get::<Fruit>("fruit_to_draw") {
        println!("I will draw {}!", arg.0);
        if let Ok(size) = args.get::<f32>("size") {
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
