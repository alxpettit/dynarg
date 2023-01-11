use indexmap::IndexMap;
use num::Float;
use num::Integer;
use std::any::Any;
use std::borrow::BorrowMut;
use std::fmt::Debug;
use std::fmt::{Display, Formatter};
use tracing::info;
use tracing::warn;

struct ArgData(Box<dyn Any>);

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

    fn get<T>(&mut self) -> Option<&T>
    where
        T: 'static,
    {
        match self.data.0.downcast_ref::<T>() {
            Some(value) => {
                self.used = true;
                Some(value)
            }
            None => None,
        }
    }

    fn used(&self) -> bool {
        self.used
    }
}

#[derive(Default)]
pub struct Args(IndexMap<String, Arg>);

impl Args {
    fn new() -> Self {
        Self::default()
    }
    pub fn get<T>(&mut self, name: &str) -> Option<&T>
    where
        T: 'static,
    {
        match self.0.get_mut(name) {
            None => None,
            Some(value) => value.get::<T>(),
        }
    }
    //
    // fn get_string(&mut self, name: &str) -> Option<&str> {
    //     // if let Some(v) = self.0.get::<String>(&name) {
    //     //     Some(v)
    //     // }
    //     if let Some(v) = self.0.get::<String>(name) {
    //         //println!("{:#?}", v);
    //     }
    //     None
    // }

    pub fn insert<S>(&mut self, name: S, value: Box<dyn Any>)
    where
        S: Into<String>,
    {
        self.0.insert(name.into(), Arg::new(value));
    }

    pub fn all_used(&self) -> bool {
        for (_arg_name, arg) in &self.0 {
            if arg.used {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arg() {
        let a = 5;
        let mut arg = Arg::new(Box::new(a));
        assert_eq!(arg.get::<i32>(), Some(&5));
        assert_eq!(arg.used(), true);

        let test = "apple";
        let mut arg = Arg::new(Box::new(test));
        assert_eq!(arg.get::<&str>(), Some(&"apple"));

        let test2 = String::from("apple");
        let mut arg2 = Arg::new(Box::new(test2));
        assert_eq!(arg2.get::<String>(), Some(&"apple".to_string()));
    }

    #[test]
    fn test_args() {
        let mut args = Args::default();
        args.insert("number".to_string(), Box::new(6));
        let result = args.get::<i32>("number");
        assert_eq!(result, Some(&6));

        let static_str = "A";
        args.insert("letter", Box::new(static_str));
        // Note! This actually produces a &&str,
        // as otherwise we would violate the Sized requirement on downcast_ref()
        let arg = args.get::<&str>("letter");
        assert_eq!(arg, Some(&"A"));
    }
}
