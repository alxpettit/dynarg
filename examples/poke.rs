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
