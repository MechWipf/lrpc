//! # Examples
//!
//! ```no_run
//! use lrpc::{fun, ByteQue, Fun, Result, Store};
//!
//! fn plus_one(q: &mut ByteQue) -> ByteQue {
//!     let x = i32::restore(q);
//!     let z = x + 1;
//!     let mut r = ByteQue::new();
//!     Ok(z).store(&mut r);
//!     r
//! }
//!
//! let mut fun = Fun::new();
//! fun.regist("plus_one", plus_one);
//! let rst = Result::<i32>::restore(&mut fun.invoke(&mut fun!("plus_one", 11)));
//! assert_eq!(rst, Ok(12));
//! ```

use crate::val::{ByteQue, Store};
use std::collections::HashMap;

pub type Result<T> = std::result::Result<T, String>;

impl<T> Store for Result<T>
where
    T: Store,
{
    #[inline]
    fn store(&self, q: &mut ByteQue) {
        match self {
            Ok(t) => {
                false.store(q);
                t.store(q);
            }
            Err(e) => {
                true.store(q);
                e.store(q);
            }
        }
    }
    #[inline]
    fn restore(q: &mut ByteQue) -> Self {
        if bool::restore(q) {
            return Err(Store::restore(q));
        }
        Ok(Store::restore(q))
    }
}

pub struct Fun {
    fun: HashMap<String, fn(&mut ByteQue) -> ByteQue>,
}

impl Fun {
    pub fn new() -> Self {
        Fun {
            fun: HashMap::new(),
        }
    }

    pub fn regist(&mut self, name: &str, f: fn(&mut ByteQue) -> ByteQue) {
        self.fun.insert(String::from(name), f);
    }

    pub fn invoke(&self, q: &mut ByteQue) -> ByteQue {
        let name = String::restore(q);
        match self.fun.get(&name) {
            Some(f) => f(q),
            None => {
                let mut r = ByteQue::new();
                true.store(&mut r);
                format!("{} function not found", name).store(&mut r);
                r
            }
        }
    }
}

#[macro_export]
macro_rules! fun {
    ($name:expr $(,$arg:expr)*) => {{
        let mut q = $crate::ByteQue::new();
        String::from($name).store(&mut q);
        $($arg.store(&mut q);)*
        q
    }};
    ($name:expr, $($arg:expr,)*) => {
        $crate::fun!($name $(,$arg)*)
    }
}
