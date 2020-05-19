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

#[derive(Debug)]
pub struct ByteQue {
    buf: Vec<u8>,
    head: usize,
}

impl ByteQue {
    #[inline]
    pub fn new() -> Self {
        ByteQue {
            buf: Vec::new(),
            head: 0,
        }
    }
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        ByteQue {
            buf: Vec::with_capacity(capacity),
            head: 0,
        }
    }
    #[inline]
    pub fn reserve(&mut self, len: usize) {
        let count = self.len();
        if count <= self.head {
            let (left, right) = self.buf.split_at_mut(self.head);
            left[..count].copy_from_slice(right);
            self.head = 0;
            self.buf.truncate(count);
        }
        self.buf.reserve(len);
    }
    #[inline]
    pub fn push(&mut self, value: u8) {
        self.reserve(1);
        self.buf.push(value);
    }
    #[inline]
    pub fn pop(&mut self) -> u8 {
        if self.len() == 0 {
            0
        } else {
            let x = self.head;
            self.head += 1;
            self.buf[x]
        }
    }
    #[inline]
    pub fn push_slice(&mut self, value: &[u8]) {
        self.reserve(value.len());
        self.buf.extend_from_slice(value);
    }
    #[inline]
    pub fn pop_slice(&mut self, len: usize) -> &[u8] {
        let count = self.len();
        if count < len {
            if count == 0 {
                self.head = 0;
                self.buf.clear();
            }
            self.reserve(len - count);
            self.buf.resize(self.head + len, 0);
        }
        let x = self.head;
        self.head += len;
        &self.buf[x..self.head]
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.buf.len() - self.head
    }
}

impl From<Vec<u8>> for ByteQue {
    #[inline]
    fn from(other: Vec<u8>) -> Self {
        ByteQue {
            buf: other,
            head: 0,
        }
    }
}

impl From<ByteQue> for Vec<u8> {
    #[inline]
    fn from(mut other: ByteQue) -> Self {
        if other.head != 0 {
            let count = other.len();
            if count != 0 {
                other.buf.copy_within(other.head.., 0);
            }
            other.buf.truncate(count);
        }
        other.buf
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

macro_rules! number_store {
    ($typ:ty, $num:expr) => {
        impl Store for $typ {
            #[inline]
            fn store(&self, q: &mut ByteQue) {
                let mut s: [u8; $num] = unsafe { std::mem::transmute(*self) };
                if cfg!(target_endian = "big") {
                    s.reverse();
                }
                q.push_slice(&s);
            }
            #[inline]
            fn restore(q: &mut ByteQue) -> Self {
                let mut s = [0u8; $num];
                s.copy_from_slice(q.pop_slice($num));
                if cfg!(target_endian = "big") {
                    s.reverse();
                }
                unsafe { std::mem::transmute(s) }
            }
        }
    };
}

number_store!(i16, 2);
number_store!(u16, 2);
number_store!(i32, 4);
number_store!(u32, 4);
number_store!(i64, 8);
number_store!(u64, 8);
number_store!(i128, 16);
number_store!(u128, 16);
number_store!(f32, 4);
number_store!(f64, 8);
number_store!(char, 4);

impl Store for String {
    #[inline]
    fn store(&self, q: &mut ByteQue) {
        let v = self.as_bytes();
        v.len().store(q);
        q.push_slice(v);
    }
    #[inline]
    fn restore(q: &mut ByteQue) -> Self {
        let s = usize::restore(q);
        if s <= q.len() {
            let v = q.pop_slice(s);
            if let Ok(v) = String::from_utf8(v.to_vec()) {
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
            return Some(T::restore(q));
        }
        None
    }
}

impl<T> Store for Box<T>
where
    T: Store,
{
    #[inline]
    fn store(&self, q: &mut ByteQue) {
        (**self).store(q);
    }
    #[inline]
    fn restore(q: &mut ByteQue) -> Self {
        Box::new(T::restore(q))
    }
}

impl<T> Store for Vec<T>
where
    T: Store,
{
    #[inline]
    fn store(&self, q: &mut ByteQue) {
        let mut s = self.len();
        s.store(q);
        if s > 0 {
            let x = q.len();
            for v in self {
                v.store(q);
                if s > 1 {
                    q.reserve((q.len() - x) * (s - 1));
                    s = 0;
                }
            }
        }
    }
    #[inline]
    fn restore(q: &mut ByteQue) -> Self {
        let s = usize::restore(q);
        if s <= q.len() {
            let mut v = Vec::with_capacity(s);
            for _ in 0..s {
                v.push(T::restore(q));
            }
            return v;
        }
        // the data must be wrong
        Vec::new()
    }
}

impl<K, V> Store for std::collections::HashMap<K, V>
where
    K: Store + Eq + core::hash::Hash,
    V: Store,
{
    #[inline]
    fn store(&self, q: &mut ByteQue) {
        let mut s = self.len();
        s.store(q);
        if s > 0 {
            let x = q.len();
            for (k, v) in self {
                k.store(q);
                v.store(q);
                if s > 1 {
                    q.reserve((q.len() - x) * (s - 1));
                    s = 0;
                }
            }
        }
    }
    #[inline]
    fn restore(q: &mut ByteQue) -> Self {
        let s = usize::restore(q);
        if s <= q.len() {
            let mut m = std::collections::HashMap::with_capacity(s);
            for _ in 0..s {
                m.insert(K::restore(q), V::restore(q));
            }
            return m;
        }
        // the data must be wrong
        std::collections::HashMap::new()
    }
}

macro_rules! tuple_store {
    ($(($n:tt, $T:ident)),+) => {
        impl<$($T),+> Store for ($($T,)+)
        where
            $($T: Store),+
        {
            #[inline]
            fn store(&self, q: &mut ByteQue) {
                $(self.$n.store(q);)+
            }
            #[inline]
            fn restore(q: &mut ByteQue) -> Self {
                ($($T::restore(q),)+)
            }
        }
    };
}

tuple_store!((0, A));
tuple_store!((0, A), (1, B));
tuple_store!((0, A), (1, B), (2, C));
tuple_store!((0, A), (1, B), (2, C), (3, D));
tuple_store!((0, A), (1, B), (2, C), (3, D), (4, E));
tuple_store!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F));
tuple_store!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G));
tuple_store!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H));
tuple_store!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H), (8, I));
tuple_store!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H), (8, I), (9, J));
tuple_store!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H), (8, I), (9, J), (10, K));
tuple_store!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H), (8, I), (9, J), (10, K), (11, L));
tuple_store!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H), (8, I), (9, J), (10, K), (11, L), (12, M));
tuple_store!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H), (8, I), (9, J), (10, K), (11, L), (12, M), (13, N));
tuple_store!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H), (8, I), (9, J), (10, K), (11, L), (12, M), (13, N), (14, O));
tuple_store!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H), (8, I), (9, J), (10, K), (11, L), (12, M), (13, N), (14, O), (15, P));
tuple_store!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H), (8, I), (9, J), (10, K), (11, L), (12, M), (13, N), (14, O), (15, P), (16, Q));
tuple_store!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H), (8, I), (9, J), (10, K), (11, L), (12, M), (13, N), (14, O), (15, P), (16, Q), (17, R));
tuple_store!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H), (8, I), (9, J), (10, K), (11, L), (12, M), (13, N), (14, O), (15, P), (16, Q), (17, R), (18, S));
tuple_store!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H), (8, I), (9, J), (10, K), (11, L), (12, M), (13, N), (14, O), (15, P), (16, Q), (17, R), (18, S), (19, T));
