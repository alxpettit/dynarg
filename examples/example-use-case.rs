use dynarg::{ArgData, Args};

/// Where normally you'd need to have a fixed set of arguments, each of which would be roughly fixed types
/// -- you can dynamically push arguments on the fly instead.
/// This is useful when you need a consistent function signature for different types of functions,
/// each needing different arguments
fn draw(args: &mut Args<i32, f32>) {
    if let Some(arg) = args.get_string("thing_to_draw") {
        println!("I will draw {}!", arg);
        if let Some(size) = args.get_float("size") {
            println!("with a size of {}", size);
        }
    } else {
        panic!("Nothing to draw D:");
    }
}


fn main() {
    let mut args = Args::<i32, f32>::defaults();

    // This is how you add arguments
    args.insert("thing_to_draw", ArgData::String("apple".to_owned()));
    args.insert("size", ArgData::Float(5.2));

    draw(&mut args);
    if !args.all_used() {
        println!("Warning! I didn't use all my arguments D:");
    }
    // Clear all the used flags on args
    args.reset_status();
}