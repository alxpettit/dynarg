use std::any::Any;
use std::borrow::BorrowMut;
use std::fmt::{Display, Formatter};
use tracing::warn;
use tracing::info;
use indexmap::IndexMap;
use std::fmt::Debug;
use num::Integer;
use num::Float;

struct ArgData(Box<dyn Any >);


struct Arg {
    data: ArgData,
    used: bool
}

impl Arg {
    fn new(arg: Box<dyn Any>) -> Self {
        Self {
            data: ArgData(arg),
            used: false
        }
    }

    fn from_argdata(arg_data: ArgData) -> Self {
        Self {
            data: arg_data,
            used: false
        }
    }

    fn get<T>(&self) -> Option<&T> where T: 'static {
        self.data.0.downcast_ref::<T>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arg() {
        let a = 5;
        let arg = Arg::new(Box::new(a));

        println!("{:#?}", arg.get::<i32>());
    }
}

//
//
// struct Args {
//     data: IndexMap<Arg>
// }
//
// impl Args {
//     fn get_float<F>(&self) -> F where F: Float {
//
//     }
// }