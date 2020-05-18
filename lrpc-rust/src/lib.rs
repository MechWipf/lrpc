//! Use tcp to call functions synchronously between processes
//!
//! # Examples
//!
//! ```no_run
//! use lrpc::*;
//!
//! #[derive(CommonStore, Debug)]
//! struct Point(i32, i32);
//!
//! #[derive(CommonStore, Debug)]
//! struct Circle {
//!     center: Point,
//!     radius: u32,
//! }
//!
//! #[fmt_function]
//! fn new_circle(p: Point, r: u32) -> Circle {
//!     Circle {
//!         center: p,
//!         radius: r,
//!     }
//! }
//!
//! #[fmt_function]
//! fn circle_area(c: Circle) -> f64 {
//!     let f_radius = c.radius as f64;
//!     f_radius * f_radius * 3.14159
//! }
//!
//! fn main() {
//!     let mut srv_fun = Fun::new();
//!     srv_fun.regist("new_circle", new_circle);
//!     srv_fun.regist("circle_area", circle_area);
//!
//!     //start service
//!     std::thread::spawn(move || {
//!         service(srv_fun, "0.0.0.0:9009");
//!     });
//!     std::thread::sleep(std::time::Duration::from_millis(10));
//!
//!     let mut conn = Connection::new("127.0.0.1:9009");
//!     let circle: Result<Circle> = conn.invoke(fun!("new_circle", Point(400, 300), 100));
//!     if let Ok(circle) = circle {
//!         println!("{:?}", circle);
//!         let area: Result<f64> = conn.invoke(fun!("circle_area", circle));
//!         println!("{:?}", area);
//!     }
//! }
//! ```

mod val;
pub use val::{ByteQue, Store};
#[macro_use]
mod fun;
pub use fun::{Fun, Result};
pub use lrpc_macros::{fmt_function, CommonStore};
mod buf;
pub use buf::{send_data, RecvBuf};
mod tcp;
pub use tcp::{service, Connection};

#[cfg(test)]
mod tests;
