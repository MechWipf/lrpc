use crate::{
    buf::{send_data, RecvBuf},
    fun::{Fun, Result},
    val::{ByteQue, Store},
};
use std::{
    io::{Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
    sync::Arc,
    thread,
};

/// Use tcp in the standard library to receive data,
/// call the function with Fun,
/// this is a blocking function
pub fn service(srv_fun: Fun, addr: &str) {
    let srv_fun = Arc::new(srv_fun);
    let listener = TcpListener::bind(addr).unwrap();
    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let srv_fun = srv_fun.clone();
            thread::spawn(move || {
                let mut buf = [0u8; 1024];
                loop {
                    let mut recv = RecvBuf::new();
                    loop {
                        match recv.size() {
                            Some(s) if s == recv.len() => break,
                            _ => match stream.read(&mut buf) {
                                Ok(l) if l > 0 => recv.append(&buf[..l]),
                                _ => match stream.shutdown(Shutdown::Both) {
                                    _ => return,
                                },
                            },
                        }
                    }
                    if let Err(_) = stream.write_all(&send_data(srv_fun.invoke(&mut recv.into()))) {
                        match stream.shutdown(Shutdown::Both) {
                            _ => return,
                        }
                    }
                }
            });
        }
    }
}

pub struct Connection(TcpStream);

impl Connection {
    pub fn new(addr: &str) -> Self {
        Connection(TcpStream::connect(addr).unwrap())
    }

    /// Use tcp in the standard library to send data.
    /// The returned result must be of type Result.
    /// If the return value of the calling function is of type Result,
    /// it will be reassembled.
    pub fn invoke<T: Store>(&mut self, fun: ByteQue) -> Result<T> {
        if let Err(e) = self.0.write_all(&send_data(fun)) {
            return Err(format!("{}", e));
        }
        let mut recv = RecvBuf::new();
        let mut buf = [0u8; 1024];
        loop {
            match recv.size() {
                Some(s) if s == recv.len() => break,
                _ => match self.0.read(&mut buf) {
                    Ok(l) => {
                        if l > 0 {
                            recv.append(&buf[..l]);
                        } else {
                            return Err(String::from("the server is disconnected"));
                        }
                    }
                    Err(e) => return Err(format!("{}", e)),
                },
            }
        }
        Store::restore(&mut recv.into())
    }
}
