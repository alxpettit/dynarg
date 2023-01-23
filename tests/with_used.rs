#[cfg(test)]
mod tests {
    use dynarg::DynArgError::{NoSuchArg, NotOfType};
    use dynarg::{Arg, Args};

    #[test]
    fn test_arg() {
        let a = 5;
        let mut arg = Arg::new(Box::new(a));
        #[cfg(feature = "used")]
        assert_eq!(arg.poke::<i32>(), Ok(&5));
        #[cfg(feature = "used")]
        assert_eq!(arg.used(), true);

        let arg = Arg::new(Box::new(a));
        #[cfg(feature = "used")]
        assert_eq!(arg.used(), false);

        let test = "apple";
        let arg = Arg::new(Box::new(test));
        assert_eq!(arg.get::<&str>(), Ok(&"apple"));

        let test2 = String::from("apple");
        let arg2 = Arg::new(Box::new(test2));
        assert_eq!(arg2.get::<String>(), Ok(&"apple".to_string()));

        let test3 = String::from("apple");
        let arg3 = Arg::new(Box::new(test3));
        assert_eq!(arg3.get::<i32>(), Err(NotOfType { name: "i32" }));
    }

    #[test]
    fn test_args() {
        let mut args = Args::default();
        let arg_name = String::from("number");
        args.insert(arg_name.as_str(), Box::new(6));
        let result = args.get::<i32>("number");
        assert_eq!(result, Ok(&6));

        let static_str = "A";
        args.insert("letter", Box::new(static_str));
        let arg = args.get::<&str>("letter");
        assert_eq!(arg, Ok(&"A"));

        let arg2 = args.get::<&str>("nonexistent");

        assert_eq!(
            arg2,
            Err(NoSuchArg {
                name: "nonexistent"
            })
        );

        let mut args = Args::default();
        args.insert_i32("nice", 69);
        args.insert_i32("wow", 42);
        #[cfg(feature = "used")]
        args.poke_i32("nice").unwrap();
        #[cfg(feature = "used")]
        assert_eq!(args.all_used(), false);
        #[cfg(feature = "used")]
        assert_eq!(args.iter_not_used_name().collect::<Vec<&str>>(), ["wow"]);

        let mut args = Args::default();
        args.insert_i32("nice", 69);
        args.insert_i32("wow", 42);
        #[cfg(feature = "used")]
        args.poke_i32("nice").unwrap();
        #[cfg(feature = "used")]
        assert_eq!(args.all_used(), false);
        #[cfg(feature = "used")]
        assert_eq!(args.iter_used_name().collect::<Vec<&str>>(), ["nice"]);

        let mut args = Args::default();
        let mut name = String::new();
        name.push_str("henlo");
        args.insert_i32(name.as_str(), 56);
        #[cfg(feature = "used")]
        assert_eq!(args.poke_i32(name.as_str()).unwrap(), &56);
    }
}
