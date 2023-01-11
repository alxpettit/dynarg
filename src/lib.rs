use crate::DynArgError::{NoSuchArg, NotOfType};
use indexmap::IndexMap;
use snafu::prelude::*;
use std::any::{type_name, Any};

struct ArgData(Box<dyn Any>);

#[derive(Debug, Snafu, PartialEq)]
pub enum DynArgError<'a> {
    #[snafu(display("No such arg: {}", name))]
    NoSuchArg { name: &'a str },

    #[snafu(display("Not of type: {}", name))]
    NotOfType { name: &'a str },
}

struct Arg {
    data: ArgData,
    used: bool,
}

impl Arg {
    fn new(arg: Box<dyn Any>) -> Self {
        Self {
            data: ArgData(arg),
            used: false,
        }
    }

    fn from_argdata(arg_data: ArgData) -> Self {
        Self {
            data: arg_data,
            used: false,
        }
    }

    fn get<T>(&mut self) -> Result<&T, DynArgError>
    where
        T: 'static,
    {
        match self.data.0.downcast_ref::<T>() {
            Some(value) => {
                self.used = true;
                Ok(value)
            }
            None => Err(NotOfType {
                name: type_name::<T>(),
            }),
        }
    }

    fn used(&self) -> bool {
        self.used
    }
}

#[derive(Default)]
pub struct Args<'a>(IndexMap<&'a str, Arg>);

macro_rules! insert_get_fn {
    ($insert_fn:ident, $get_fn:ident, $t:ty) => {
        pub fn $insert_fn(&mut self, name: &'a str, value: $t) {
            self.0.insert(name, Arg::new(Box::new(value)));
        }

        pub fn $get_fn(&mut self, name: &'a str) -> Result<&$t, DynArgError> {
            self.get::<$t>(name)
        }
    };
}

impl<'a> Args<'a> {
    fn new() -> Self {
        Self::default()
    }
    pub fn get<T>(&mut self, name: &'a str) -> Result<&T, DynArgError>
    where
        T: 'static,
    {
        match self.0.get_mut(name) {
            None => Err(NoSuchArg { name }),
            Some(arg) => Ok(arg.get::<T>()?),
        }
    }

    pub fn insert(&mut self, name: &'a str, value: Box<dyn Any>) {
        self.0.insert(name, Arg::new(value));
    }

    pub fn all_used(&self) -> bool {
        for (_arg_name, arg) in &self.0 {
            if !arg.used {
                return false;
            }
        }
        return true;
    }

    pub fn reset_status(&mut self) {
        for (_arg_name, arg) in &mut self.0 {
            arg.used = false;
        }
    }

    insert_get_fn!(insert_string, get_string, String);
    insert_get_fn!(insert_f32, get_f32, f32);
    insert_get_fn!(insert_f64, get_f64, f64);
    insert_get_fn!(insert_i32, get_i32, i32);
    insert_get_fn!(insert_i64, get_i64, i64);
    insert_get_fn!(insert_bool, get_bool, bool);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arg() {
        let a = 5;
        let mut arg = Arg::new(Box::new(a));
        assert_eq!(arg.get::<i32>(), Ok(&5));
        assert_eq!(arg.used(), true);

        let test = "apple";
        let mut arg = Arg::new(Box::new(test));
        assert_eq!(arg.get::<&str>(), Ok(&"apple"));

        let test2 = String::from("apple");
        let mut arg2 = Arg::new(Box::new(test2));
        assert_eq!(arg2.get::<String>(), Ok(&"apple".to_string()));

        let test3 = String::from("apple");
        let mut arg3 = Arg::new(Box::new(test3));
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
        // Note! This actually produces a &&str,
        // as otherwise we would violate the Sized requirement on downcast_ref()
        let arg = args.get::<&str>("letter");
        assert_eq!(arg, Ok(&"A"));

        let arg2 = args.get::<&str>("nonexistent");

        assert_eq!(
            arg2,
            Err(NoSuchArg {
                name: "nonexistent"
            })
        );
    }
}
