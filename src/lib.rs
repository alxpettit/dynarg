#[forbid(unsafe_code)]
use crate::DynArgError::{NoSuchArg, NotOfType};
use indexmap::IndexMap;
use snafu::prelude::*;
use std::any::{type_name, Any};

#[derive(Debug)]
/// A struct for wrapping base argument data, of any type.
pub struct ArgData(Box<dyn Any>);

#[derive(Debug, Snafu, PartialEq)]
/// An error struct for passing errors up the callchain.
pub enum DynArgError<'a> {
    #[snafu(display("No such arg: {}", name))]
    NoSuchArg { name: &'a str },

    #[snafu(display("Not of type: {}", name))]
    NotOfType { name: &'a str },
}

/// For storing a single argument.
/// Stores a `Box<>` to argument variable, as well as a `used` flag.
pub struct Arg {
    data: ArgData,
    #[cfg(feature = "used")]
    used: bool,
}

impl Arg {
    /// Create new Arg struct, moving ownership of Box pointer
    /// to an internal dynamically typed Any-trait variable.
    pub fn new(arg: Box<dyn Any>) -> Self {
        Self {
            data: ArgData(arg),
            #[cfg(feature = "used")]
            used: false,
        }
    }

    /// Like `new()`, but accepts an `ArgData()` instead of a raw `Box<dyn Any>`
    pub fn from_argdata(arg_data: ArgData) -> Self {
        Self {
            data: arg_data,
            #[cfg(feature = "used")]
            used: false,
        }
    }

    #[cfg(feature = "used")]
    /// Like `get()`, but marks the value as `used`.
    /// Because it changes `self`, it requires mutable access to self.
    /// This, of course, may make it unusable for some use cases.
    pub fn poke<T>(&mut self) -> Result<&T, DynArgError>
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

    /// Retrieve a struct's inner value.
    /// It's recommended to _explicitly_ specify type, e.g.:
    /// ```rust
    /// use dynarg::Arg;
    /// let mut arg = Arg::new(Box::new(42i32));
    /// let arg = arg.get::<i32>();
    /// ```
    pub fn get<T>(&self) -> Result<&T, DynArgError>
    where
        T: 'static,
    {
        match self.data.0.downcast_ref::<T>() {
            Some(value) => Ok(value),
            None => Err(NotOfType {
                name: type_name::<T>(),
            }),
        }
    }

    #[cfg(feature = "used")]
    pub fn used(&self) -> bool {
        self.used
    }
}

#[derive(Default)]
/// Stores an `IndexMap` of `Args`. See examples.
pub struct Args<'a>(IndexMap<&'a str, Arg>);

macro_rules! insert_get_fn {
    ($insert_fn:ident, $get_fn:ident, $poke_fn: ident, $t:ty) => {
        /// Dynamically generated inserter
        pub fn $insert_fn(&mut self, name: &'a str, value: $t) {
            self.0.insert(name, Arg::new(Box::new(value)));
        }

        /// Dynamically generated getter
        pub fn $get_fn(&mut self, name: &'a str) -> Result<&$t, DynArgError> {
            self.get::<$t>(name)
        }

        #[cfg(feature = "used")]
        /// Dynamically generated poker
        pub fn $poke_fn(&mut self, name: &'a str) -> Result<&$t, DynArgError> {
            self.poke::<$t>(name)
        }
    };
}

impl<'a> Args<'a> {
    /// A convenience function for making a new empty `Args()`.
    /// In truth, it just calls `default()`
    pub fn new() -> Self {
        Self::default()
    }

    /// Initializes internal hashmap with a given capacity, to reduce required memory allocations.
    /// Potentially useful if you're planning to call `.push()` _a lot_.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(IndexMap::with_capacity(capacity))
    }

    #[cfg(feature = "used")]
    /// Like `get()`, but marks the value as `used`.
    /// Because it changes `self`, it requires mutable access to self.
    /// This, of course, may make it unusable for some use cases.
    pub fn poke<T>(&mut self, name: &'a str) -> Result<&T, DynArgError>
    where
        T: 'static,
    {
        match self.0.get_mut(name) {
            None => Err(NoSuchArg { name }),
            Some(arg) => Ok(arg.poke::<T>()?),
        }
    }

    /// Retrieve a value by name. It's recommended to _explicitly_ specify type via generics, e.g.:
    /// ```rust
    /// use dynarg::Args;
    /// let mut args = Args::default();
    /// args.insert_i32("meaning_of_life", 42);
    /// let arg = args.get::<i32>("meaning_of_life");
    /// ```
    pub fn get<T>(&self, name: &'a str) -> Result<&T, DynArgError>
    where
        T: 'static,
    {
        match self.0.get(name) {
            None => Err(NoSuchArg { name }),
            Some(arg) => Ok(arg.get::<T>()?),
        }
    }

    /// Inserts a value with a dynamic type. Value must be wrapped in a `Box<>` pointer.
    /// BTW, when will the `box` keyword syntax be stable? That would make a lot of code more elegant.
    pub fn insert(&mut self, name: &'a str, value: Box<dyn Any>) {
        self.0.insert(name, Arg::new(value));
    }

    #[cfg(feature = "used")]
    /// Returns true if every argument is marked as "used". Returns false otherwise.
    pub fn all_used(&self) -> bool {
        self.iter().map(|x| x.1.used).all(|b| b)
    }

    #[cfg(feature = "used")]
    /// Returns a iterator of arguments marked as not used.
    pub fn iter_not_used_name(&self) -> impl Iterator<Item = &str> {
        self.iter().filter_map(|x| {
            let (arg_name, arg) = x;
            match arg.used {
                false => Some(arg_name),
                true => None,
            }
        })
    }

    #[cfg(feature = "used")]
    /// Returns an iterator of arguments marked as used.
    pub fn iter_used_name(&self) -> impl Iterator<Item = &str> {
        self.iter().filter_map(|x| {
            let (arg_name, arg) = x;
            match arg.used {
                true => Some(arg_name),
                false => None,
            }
        })
    }

    #[cfg(feature = "used")]
    /// Resets the used status of every argument in the IndexMap.
    pub fn reset_used_status(&mut self) {
        for (_arg_name, arg) in &mut self.0 {
            arg.used = false;
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &Arg)> {
        // iter() returns &&str normally,
        // but we want &str, so we have to deref the nested pointer Y~Y
        self.0.iter().map(|x| {
            let (arg_name, arg) = x;
            (*arg_name, arg)
        })
    }

    insert_get_fn!(insert_string, get_string, poke_string, String);
    insert_get_fn!(insert_f32, get_f32, poke_f32, f32);
    insert_get_fn!(insert_f64, get_f64, poke_f64, f64);
    insert_get_fn!(insert_i32, get_i32, poke_i32, i32);
    insert_get_fn!(insert_i64, get_i64, poke_i64, i64);
    insert_get_fn!(insert_bool, get_bool, poke_bool, bool);
}