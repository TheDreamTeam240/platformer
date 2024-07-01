use std::net::{TcpListener, TcpStream};
use std::net::{Ipv4Addr, SocketAddrV4};

const ADDR: Ipv4Addr = Ipv4Addr::LOCALHOST;

pub struct Server {
    port: u16,
    fd: TcpListener,
    is_running: bool
}

impl Server {
    pub fn new() -> Server {
        Server {
            port: 0,
            fd: TcpListener::bind(SocketAddrV4::new(ADDR, 0)).unwrap(),
            is_running: false
        }
    }

    pub fn init(&mut self, port: u16) {
        self.port = port;
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub fn run(&mut self) {
        println!("Server is running on port {}", self.port);
        self.fd = TcpListener::bind(SocketAddrV4::new(ADDR, self.port)).unwrap();
        self.is_running = true;
    }

    pub fn isRunning(&self) -> bool {
        self.is_running
    }
}

