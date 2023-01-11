use std::borrow::BorrowMut;
use std::fmt::{Display, Formatter};
use tracing::warn;
use tracing::info;
use indexmap::IndexMap;
use std::fmt::Debug;
use num::Integer;

/// An enum representing any argument data we might have
/// TODO: figure out some way of dynamically handling arbitrary types
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum ArgData<I> where I: Integer, F {
    String(String),
    Integer(I),
    Bool(bool),
    #[default]
    Undefined
}

impl <I>Display for ArgData<I> where I: Integer + Debug {
    fn fmt(self: &ArgData<I>, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(&mut f.borrow_mut(), "{:#?}", self)
    }
}

/// A struct representing an argument, holding both the `ArgData` data itself and a `used` state
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct Arg<I> where I: Integer {
    pub data: ArgData<I>,
    pub used: bool
}

impl <I>Arg<I> where I: Default + Integer {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn from_argdata(arg_data: ArgData<I>) -> Self {
        Self {
            data: arg_data,
            used: false
        }

    }
}

impl <I>Display for Arg<I> where I: Integer + Clone + Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(&mut f.borrow_mut(), "{}", self.data.clone())
    }
}

macro_rules! generate_get {
    ($func:ident, $args:ident, $data_type:ident, $arg_data_type:path) => {
        /// Data-type specific getter
        pub fn $func(&mut $args, arg_name: &str) -> Option<$data_type> {
            match &$args.get(arg_name)?.data {
                $arg_data_type(arg) => { Some(arg.clone()) },
                _ => { warn!("Attempted to get \"{}\" but could not.", arg_name); None }
            }
        }
    }
}


#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Args<I> where I: Integer {
    pub args: IndexMap<String, Arg<I>>
}


/// A `Vec` of arguments, representing a collection of any data you might find in `ArgData`
impl <I>Args<I> where I: Integer + Clone + Debug + Default {

    /// Get an argument from its lookup string
    pub fn get(&mut self, string: &str) -> Option<&Arg<I>> {
        let arg = self.args.get(string);
        if arg.is_none() {
            info!("Attempted to get \"{}\", but could not", string);
        }
        arg
    }

    // TODO: replace with macro that can dynamically infer types from
    generate_get!(get_string, self, String, ArgData::String);
    generate_get!(get_i32, self, I, ArgData::Integer);
    generate_get!(get_bool, self, bool, ArgData::Bool);

    pub fn defaults() -> Self {
        Self {
            args: IndexMap::<String, Arg<I>>::new()
        }
    }

    pub fn all_used(&self) -> bool {
        for (_arg_name, arg) in &self.args {
            if arg.used == false {
                return false;
            }
        }
        return true;
    }

    pub fn reset_status(&mut self) {
        for (_arg_name, arg) in &mut self.args {
            arg.used = false;
        }
    }

    #[inline]
    pub fn insert(&mut self, arg_name: &str, arg_data: ArgData<I>) {
        self.args.insert(arg_name.to_owned(), Arg::from_argdata(arg_data));
    }


    #[inline]
    pub fn insert_arg(&mut self, arg_name: &str, arg: Arg<I>) {
        self.args.insert(arg_name.to_owned(), arg);
    }

    #[inline]
    pub fn remove(&mut self, arg_name: &str) {
        self.args.remove(arg_name);
    }
}