//! # Examples
//!
//! ```no_run
//! use lrpc::{ByteQue, Store};
//!
//! struct Points {
//!     x: [i32; 6],
//!     y: [i32; 6],
//! }
//!
//! impl Store for Points {
//!     fn store(&self, q: &mut ByteQue) {
//!         for i in 0..6 {
//!             self.x[i].store(q);
//!         }
//!         for i in 0..6 {
//!             self.y[i].store(q);
//!         }
//!     }
//!     fn restore(q: &mut ByteQue) -> Self {
//!         let mut x = [0; 6];
//!         let mut y = [0; 6];
//!         for i in 0..6 {
//!             x[i] = Store::restore(q);
//!         }
//!         for i in 0..6 {
//!             y[i] = Store::restore(q);
//!         }
//!         Points { x, y }
//!     }
//! }
//! ```

use std::{collections::VecDeque, mem::transmute};

#[derive(Debug)]
pub struct ByteQue(VecDeque<u8>);

impl ByteQue {
    #[inline]
    pub fn new() -> Self {
        ByteQue(VecDeque::new())
    }
    #[inline]
    pub fn push(&mut self, value: u8) {
        self.0.push_back(value);
    }
    #[inline]
    pub fn pop(&mut self) -> u8 {
        self.0.pop_front().unwrap_or(0)
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl From<Vec<u8>> for ByteQue {
    #[inline]
    fn from(other: Vec<u8>) -> Self {
        ByteQue(VecDeque::from(other))
    }
}

impl From<ByteQue> for Vec<u8> {
    #[inline]
    fn from(other: ByteQue) -> Self {
        Vec::<u8>::from(other.0)
    }
}

pub trait Store {
    fn store(&self, q: &mut ByteQue);
    fn restore(q: &mut ByteQue) -> Self;
}

impl Store for usize {
    #[inline]
    fn store(&self, q: &mut ByteQue) {
        let mut s = *self;
        // maximum number of 64-bit computers
        for _ in 0..10 {
            if s <= 0x7f {
                q.push(s as u8 & 0x7f);
                break;
            } else {
                q.push(s as u8 & 0x7f | 0x80);
            }
            s >>= 7;
        }
    }
    #[inline]
    fn restore(q: &mut ByteQue) -> Self {
        let mut s = 0usize;
        // 64-bit computers will overflow if this number is exceeded
        for i in 0..10 {
            let v = q.pop();
            s |= (v as usize & 0x7f) << 7 * i;
            if v <= 0x7f {
                break;
            }
        }
        s
    }
}

impl Store for i8 {
    #[inline]
    fn store(&self, q: &mut ByteQue) {
        q.push(*self as u8);
    }
    #[inline]
    fn restore(q: &mut ByteQue) -> Self {
        q.pop() as i8
    }
}

impl Store for i16 {
    #[inline]
    fn store(&self, q: &mut ByteQue) {
        q.push(*self as u8);
        q.push((*self >> 8) as u8);
    }
    #[inline]
    fn restore(q: &mut ByteQue) -> Self {
        q.pop() as i16 | (q.pop() as i16) << 8
    }
}

impl Store for i32 {
    #[inline]
    fn store(&self, q: &mut ByteQue) {
        q.push(*self as u8);
        q.push((*self >> 8) as u8);
        q.push((*self >> 16) as u8);
        q.push((*self >> 24) as u8);
    }
    #[inline]
    fn restore(q: &mut ByteQue) -> Self {
        q.pop() as i32 | (q.pop() as i32) << 8 | (q.pop() as i32) << 16 | (q.pop() as i32) << 24
    }
}

impl Store for i64 {
    #[inline]
    fn store(&self, q: &mut ByteQue) {
        q.push(*self as u8);
        q.push((*self >> 8) as u8);
        q.push((*self >> 16) as u8);
        q.push((*self >> 24) as u8);
        q.push((*self >> 32) as u8);
        q.push((*self >> 40) as u8);
        q.push((*self >> 48) as u8);
        q.push((*self >> 56) as u8);
    }
    #[inline]
    fn restore(q: &mut ByteQue) -> Self {
        q.pop() as i64
            | (q.pop() as i64) << 8
            | (q.pop() as i64) << 16
            | (q.pop() as i64) << 24
            | (q.pop() as i64) << 32
            | (q.pop() as i64) << 40
            | (q.pop() as i64) << 48
            | (q.pop() as i64) << 56
    }
}

impl Store for i128 {
    #[inline]
    fn store(&self, q: &mut ByteQue) {
        q.push(*self as u8);
        q.push((*self >> 8) as u8);
        q.push((*self >> 16) as u8);
        q.push((*self >> 24) as u8);
        q.push((*self >> 32) as u8);
        q.push((*self >> 40) as u8);
        q.push((*self >> 48) as u8);
        q.push((*self >> 56) as u8);
        q.push((*self >> 64) as u8);
        q.push((*self >> 72) as u8);
        q.push((*self >> 80) as u8);
        q.push((*self >> 88) as u8);
        q.push((*self >> 96) as u8);
        q.push((*self >> 104) as u8);
        q.push((*self >> 112) as u8);
        q.push((*self >> 120) as u8);
    }
    #[inline]
    fn restore(q: &mut ByteQue) -> Self {
        q.pop() as i128
            | (q.pop() as i128) << 8
            | (q.pop() as i128) << 16
            | (q.pop() as i128) << 24
            | (q.pop() as i128) << 32
            | (q.pop() as i128) << 40
            | (q.pop() as i128) << 48
            | (q.pop() as i128) << 56
            | (q.pop() as i128) << 64
            | (q.pop() as i128) << 72
            | (q.pop() as i128) << 80
            | (q.pop() as i128) << 88
            | (q.pop() as i128) << 96
            | (q.pop() as i128) << 104
            | (q.pop() as i128) << 112
            | (q.pop() as i128) << 120
    }
}

impl Store for f32 {
    #[inline]
    fn store(&self, q: &mut ByteQue) {
        unsafe { transmute::<f32, i32>(*self).store(q) };
    }
    #[inline]
    fn restore(q: &mut ByteQue) -> Self {
        unsafe { transmute::<i32, f32>(i32::restore(q)) }
    }
}

impl Store for f64 {
    #[inline]
    fn store(&self, q: &mut ByteQue) {
        unsafe { transmute::<f64, i64>(*self).store(q) }
    }
    #[inline]
    fn restore(q: &mut ByteQue) -> Self {
        unsafe { transmute::<i64, f64>(i64::restore(q)) }
    }
}

impl Store for bool {
    #[inline]
    fn store(&self, q: &mut ByteQue) {
        q.push(if *self { 1u8 } else { 0u8 });
    }
    #[inline]
    fn restore(q: &mut ByteQue) -> Self {
        q.pop() != 0
    }
}

impl Store for u8 {
    #[inline]
    fn store(&self, q: &mut ByteQue) {
        q.push(*self);
    }
    #[inline]
    fn restore(q: &mut ByteQue) -> Self {
        q.pop()
    }
}

impl Store for u16 {
    #[inline]
    fn store(&self, q: &mut ByteQue) {
        q.push(*self as u8);
        q.push((*self >> 8) as u8);
    }
    #[inline]
    fn restore(q: &mut ByteQue) -> Self {
        q.pop() as u16 | (q.pop() as u16) << 8
    }
}

impl Store for u32 {
    #[inline]
    fn store(&self, q: &mut ByteQue) {
        q.push(*self as u8);
        q.push((*self >> 8) as u8);
        q.push((*self >> 16) as u8);
        q.push((*self >> 24) as u8);
    }
    #[inline]
    fn restore(q: &mut ByteQue) -> Self {
        q.pop() as u32 | (q.pop() as u32) << 8 | (q.pop() as u32) << 16 | (q.pop() as u32) << 24
    }
}

impl Store for u64 {
    #[inline]
    fn store(&self, q: &mut ByteQue) {
        q.push(*self as u8);
        q.push((*self >> 8) as u8);
        q.push((*self >> 16) as u8);
        q.push((*self >> 24) as u8);
        q.push((*self >> 32) as u8);
        q.push((*self >> 40) as u8);
        q.push((*self >> 48) as u8);
        q.push((*self >> 56) as u8);
    }
    #[inline]
    fn restore(q: &mut ByteQue) -> Self {
        q.pop() as u64
            | (q.pop() as u64) << 8
            | (q.pop() as u64) << 16
            | (q.pop() as u64) << 24
            | (q.pop() as u64) << 32
            | (q.pop() as u64) << 40
            | (q.pop() as u64) << 48
            | (q.pop() as u64) << 56
    }
}

impl Store for u128 {
    #[inline]
    fn store(&self, q: &mut ByteQue) {
        q.push(*self as u8);
        q.push((*self >> 8) as u8);
        q.push((*self >> 16) as u8);
        q.push((*self >> 24) as u8);
        q.push((*self >> 32) as u8);
        q.push((*self >> 40) as u8);
        q.push((*self >> 48) as u8);
        q.push((*self >> 56) as u8);
        q.push((*self >> 64) as u8);
        q.push((*self >> 72) as u8);
        q.push((*self >> 80) as u8);
        q.push((*self >> 88) as u8);
        q.push((*self >> 96) as u8);
        q.push((*self >> 104) as u8);
        q.push((*self >> 112) as u8);
        q.push((*self >> 120) as u8);
    }
    #[inline]
    fn restore(q: &mut ByteQue) -> Self {
        q.pop() as u128
            | (q.pop() as u128) << 8
            | (q.pop() as u128) << 16
            | (q.pop() as u128) << 24
            | (q.pop() as u128) << 32
            | (q.pop() as u128) << 40
            | (q.pop() as u128) << 48
            | (q.pop() as u128) << 56
            | (q.pop() as u128) << 64
            | (q.pop() as u128) << 72
            | (q.pop() as u128) << 80
            | (q.pop() as u128) << 88
            | (q.pop() as u128) << 96
            | (q.pop() as u128) << 104
            | (q.pop() as u128) << 112
            | (q.pop() as u128) << 120
    }
}

impl Store for String {
    #[inline]
    fn store(&self, q: &mut ByteQue) {
        let v = self.as_bytes();
        let l = v.len();
        l.store(q);
        for i in 0..l {
            q.push(v[i]);
        }
    }
    #[inline]
    fn restore(q: &mut ByteQue) -> Self {
        let l = usize::restore(q);
        if l <= q.len() {
            let mut v = Vec::with_capacity(l);
            for _ in 0..l {
                v.push(q.pop());
            }
            if let Ok(v) = String::from_utf8(v) {
                return v;
            }
        }
        // the data must be wrong
        String::new()
    }
}

impl Store for () {
    #[inline]
    fn store(&self, _: &mut ByteQue) {}
    #[inline]
    fn restore(_: &mut ByteQue) -> Self {
        ()
    }
}

impl<T> Store for Option<T>
where
    T: Store,
{
    #[inline]
    fn store(&self, q: &mut ByteQue) {
        match self {
            Some(t) => {
                true.store(q);
                t.store(q);
            }
            None => false.store(q),
        }
    }
    #[inline]
    fn restore(q: &mut ByteQue) -> Self {
        if bool::restore(q) {
            return Some(Store::restore(q));
        }
        None
    }
}

impl<T> Store for Vec<T>
where
    T: Store,
{
    #[inline]
    fn store(&self, q: &mut ByteQue) {
        let l = self.len();
        l.store(q);
        for i in 0..l {
            self[i].store(q);
        }
    }
    #[inline]
    fn restore(q: &mut ByteQue) -> Self {
        let l = usize::restore(q);
        if l <= q.len() {
            let mut v = Vec::with_capacity(l);
            for _ in 0..l {
                v.push(Store::restore(q));
            }
            return v;
        }
        // the data must be wrong
        Vec::new()
    }
}
