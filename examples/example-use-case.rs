use dynarg::ArgData;

/// Where normally you'd need to have a fixed set of arguments,
/// each of which would be roughly fixed types -- you can now concisely feed an enum vector instead.
fn draw(args: &mut dynarg::Args) {
    if let Some(arg) = args.get("thing_to_draw") {
        println!("I will draw {}!", arg);
    } else {
        panic!("Nothing to draw D:");
    }
}


fn main() {
    let mut args = dynarg::Args::defaults();
    args.insert("thing_to_draw", ArgData::String("apple".to_owned()));
    draw(&mut args);
    if !args.all_used() {
        println!("Warning! I didn't use all my arguments D:");
    }
    // Clear all the used flags on args
    args.reset_status();
}