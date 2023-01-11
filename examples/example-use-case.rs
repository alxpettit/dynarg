use dynarg::*;

/// Where normally you'd need to have a fixed set of arguments, each of which would be roughly fixed types
/// -- you can dynamically push arguments on the fly instead.
/// This is useful when you need a consistent function signature for different types of functions,
/// each needing different arguments
fn draw(args: &mut Args) {
    if let Some(arg) = args.get::<String>("thing_to_draw") {
        println!("I will draw {}!", arg);
        if let Some(size) = args.get::<f32>("size") {
            println!("with a size of {}", size);
        }
    } else {
        panic!("Nothing to draw D:");
    }
}

fn main() {
    let mut args = Args::default();

    // This is how you add arguments
    args.insert("thing_to_draw", Box::new("apple".to_owned()));
    args.insert("size", Box::new(5.2));

    draw(&mut args);
    if !args.all_used() {
        println!("Warning! I didn't use all my arguments D:");
    }
    // Clear all the used flags on args
    args.reset_status();
}
