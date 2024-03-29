use dynarg::Args;

/// Where normally you'd need to have a fixed set of arguments, each of which would be roughly fixed types
/// -- you can dynamically push arguments on the fly instead.
/// This is useful when you need a consistent function signature for different types of functions,
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
    // Much like Vec and others, creating `with_capacity` can prevent unnecessary
    // repeated heap allocation calls, thus improving program performance.
    let mut args = Args::with_capacity(100);

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
