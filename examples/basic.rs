use dynarg::Args;

fn main() {
    // Creating Args object
    let mut args = Args::new();
    // Inserting a string type
    args.insert_string("greeting", String::from("hello world"));

    // Retrieving
    let out = args.get_string("greeting").unwrap();
    println!("{}", out);
}
