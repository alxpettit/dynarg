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
