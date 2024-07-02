use mio::net::TcpStream;
use std::os::fd::AsRawFd;
use std::io::Write;

pub struct Connection {
    stream: Option<TcpStream>,
    buffer: Vec<u8>
}

impl  Connection {
    pub fn new() -> Connection {
        Connection {
            stream: None,
            buffer: Vec::new()
        }
    }

    pub fn setStream(&mut self, stream: TcpStream) {
        self.stream = Some(stream);
    }

    pub fn getStream(&mut self) -> Option<&mut TcpStream> {
        self.stream.as_mut()
    }

    pub fn getfd(&self) -> i32 {
        self.stream.as_ref().unwrap().as_raw_fd()
    }

    pub fn send(&mut self, data: &[u8]) {
        if let Some(ref mut stream) = self.stream {
            stream.write(data);
        }
    }
}
