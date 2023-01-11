use std::borrow::BorrowMut;
use std::fmt::{Display, Formatter};
use tracing::warn;
use tracing::info;
use indexmap::IndexMap;
use std::fmt::Debug;
use num::Integer;
use num::Float;

/// An enum representing any argument data we might have
/// TODO: figure out some way of dynamically handling arbitrary types
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum ArgData<I, F> where I: Integer, F: Float {
    String(String),
    Integer(I),
    Float(F),
    Bool(bool),
    #[default]
    Undefined
}

impl <I, F>Display for ArgData<I, F> where I: Integer + Debug, F: Float + Debug {
    fn fmt(self: &ArgData<I, F>, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(&mut f.borrow_mut(), "{:#?}", self)
    }
}

/// A struct representing an argument, holding both the `ArgData` data itself and a `used` state
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct Arg<I, F> where I: Integer, F: Float {
    pub data: ArgData<I, F>,
    pub used: bool
}

impl <I, F>Arg<I, F> where I: Default + Integer, F: Default + Float {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn from_argdata(arg_data: ArgData<I, F>) -> Self {
        Self {
            data: arg_data,
            used: false
        }

    }
}

impl <I, F>Display for Arg<I, F> where I: Integer + Clone + Debug, F: Float + Clone + Debug {
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
pub struct Args<I, F> where I: Integer, F: Float {
    pub args: IndexMap<String, Arg<I, F>>
}


/// A `Vec` of arguments, representing a collection of any data you might find in `ArgData`
impl <I, F>Args<I, F> where I: Integer + Clone + Debug + Default, F: Float + Clone + Debug + Default {

    /// Get an argument from its lookup string
    pub fn get(&mut self, string: &str) -> Option<&Arg<I, F>> {
        match self.args.get_mut(string) {
            None => {
                info!("Attempted to get \"{}\", but could not", string);
                None
            },
    
            Some(arg) => {
                arg.used = true;
                Some(arg)
            }

        }
    }

    // TODO: replace with macro that can dynamically infer types from
    generate_get!(get_string, self, String, ArgData::String);
    generate_get!(get_int, self, I, ArgData::Integer);
    generate_get!(get_float, self, F, ArgData::Float);
    generate_get!(get_bool, self, bool, ArgData::Bool);

    pub fn defaults() -> Self {
        Self {
            args: IndexMap::<String, Arg<I, F>>::new()
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
    pub fn insert(&mut self, arg_name: &str, arg_data: ArgData<I, F>) {
        self.args.insert(arg_name.to_owned(), Arg::from_argdata(arg_data));
    }


    #[inline]
    pub fn insert_arg(&mut self, arg_name: &str, arg: Arg<I, F>) {
        self.args.insert(arg_name.to_owned(), arg);
    }

    #[inline]
    pub fn remove(&mut self, arg_name: &str) {
        self.args.remove(arg_name);
    }
}