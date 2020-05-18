//! Add length to the actual data to judge the integrity of the data

use crate::val::ByteQue;

#[derive(Debug)]
pub struct RecvBuf {
    buf: Vec<u8>,
    size: Option<usize>,
}

impl RecvBuf {
    #[inline]
    pub fn new() -> Self {
        RecvBuf {
            buf: Vec::new(),
            size: None,
        }
    }

    pub fn append(&mut self, other: &[u8]) {
        match self.size {
            Some(s) => {
                if s > self.buf.len() {
                    let l = s - self.buf.len();
                    if l < other.len() {
                        self.buf.extend_from_slice(&other[..l]);
                    } else {
                        self.buf.extend_from_slice(other);
                    }
                }
            }
            None => {
                if self.buf.is_empty() {
                    for x in 0..other.len() {
                        // 64-bit computer will overflow if greater than 9
                        if x == 9 || other[x] <= 0x7f {
                            let mut s = 0usize;
                            for i in 0..=x {
                                s |= (other[i] as usize & 0x7f) << 7 * i;
                            }
                            self.size = Some(s);
                            let t = &other[x + 1..];
                            if s < t.len() {
                                self.buf.extend_from_slice(&t[..s]);
                            } else {
                                self.buf.extend_from_slice(t);
                            }
                            return;
                        }
                    }
                    self.buf.extend_from_slice(other);
                } else {
                    self.buf.extend_from_slice(other);
                    for x in 0..self.buf.len() {
                        // 64-bit computer will overflow if greater than 9
                        if x == 9 || self.buf[x] <= 0x7f {
                            let mut s = 0usize;
                            for i in 0..=x {
                                s |= (self.buf.remove(0) as usize & 0x7f) << 7 * i;
                            }
                            self.size = Some(s);
                            if self.buf.len() > s {
                                self.buf.resize(s, 0);
                            }
                            break;
                        }
                    }
                }
            }
        }
    }

    #[inline]
    pub fn size(&self) -> Option<usize> {
        self.size
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.buf.len()
    }
}

impl From<RecvBuf> for ByteQue {
    #[inline]
    fn from(other: RecvBuf) -> Self {
        ByteQue::from(other.buf)
    }
}

pub fn send_data(q: ByteQue) -> Vec<u8> {
    let mut v = Vec::new();
    let mut s = q.len();
    // maximum number of 64-bit computers
    for _ in 0..10 {
        if s <= 0x7f {
            v.push(s as u8 & 0x7f);
            break;
        } else {
            v.push(s as u8 & 0x7f | 0x80);
        }
        s >>= 7;
    }
    v.append(&mut Vec::<u8>::from(q));
    v
}
