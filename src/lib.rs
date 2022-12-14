use tracing::warn;
use tracing::info;
use indexmap::IndexMap;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum ArgData {
    String(String),
    I32(i32),
    Bool(bool),
    #[default]
    Undefined
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct Arg {
    pub data: ArgData,
    pub used: bool
}

impl Arg {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn from_argdata(arg_data: ArgData) -> Self {
        Self {
            data: arg_data,
            used: false
        }

    }
}

macro_rules! generate_get {
    ($func:ident, $args:ident, $data_type:ident, $arg_data_type:path) => {
        pub fn $func(&mut $args, arg_name: &str) -> Option<$data_type> {
            match &$args.get(arg_name)?.data {
                $arg_data_type(arg) => { Some(arg.clone()) },
                _ => { warn!("Attempted to get \"{}\" but could not.", arg_name); None }
            }
        }
    }
}


#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Args {
    pub args: IndexMap<String, Arg>
}

impl Args {
    pub fn get(&mut self, string: &str) -> Option<&Arg> {
        let arg = self.args.get(string);
        if arg.is_none() {
            info!("Attempted to get \"{}\", but could not", string);
        }
        arg
    }

    // TODO: replace with macro that can dynamically infer types from
    generate_get!(get_string, self, String, ArgData::String);
    generate_get!(get_i32, self, i32, ArgData::I32);
    generate_get!(get_bool, self, bool, ArgData::Bool);

    pub fn defaults() -> Self {
        Self {
            args: IndexMap::<String, Arg>::new()
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
}

// fn call_dynarg_func(args: &mut Arguments, func: fn (&mut Arguments)) {
//     func(args);
//     args.all_used()
// }