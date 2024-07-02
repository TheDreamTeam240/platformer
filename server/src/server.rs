use mio::net::TcpListener;
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use crate::connection::Connection;
use std::collections::HashMap;
use mio::{Events, Poll, Token};
use mio::Interest;
use std::io;

const ADDR: Ipv4Addr = Ipv4Addr::LOCALHOST;

pub struct Server {
    port: u16,
    fd: Option<TcpListener>,
    is_running: bool,
    mapConnections: HashMap<u16, Connection>,
    waitingConnections: Vec<Connection>,
    poll : Poll,
    events : Events
}

impl Server {
    pub fn new() -> Server {
        Server {
            port: 0,
            fd: None,
            is_running: false,
            mapConnections: HashMap::new(),
            waitingConnections: Vec::new(),
            poll: Poll::new().unwrap(),
            events: Events::with_capacity(1024)
        }
    }

    pub fn init(&mut self, port: u16) {
        self.port = port;
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub fn run(&mut self) -> io::Result<()> {
        println!("Server is running on port {}", self.port);
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), self.port);

        // Bind the address to the mio TcpListener
        match TcpListener::bind(addr) {
            Ok(listener) => {
                self.fd = Some(listener);
                self.is_running = true;
            }
            Err(e) => {
                eprintln!("Failed to bind to address: {}", e);
                self.is_running = false;
            }
        }
        if let Some(ref mut fd) = self.fd {
            self.poll.registry().register(fd, Token(0), Interest::READABLE | Interest::WRITABLE)?;
        }
        Ok(())
    }

    pub fn isRunning(&self) -> bool {
        self.is_running
    }

    pub fn handleNewConnection(&mut self) {
        if let Some(ref mut fd) = self.fd {
            match fd.accept() {
                Ok((stream, _)) => {
                    let mut connection = Connection::new();
                    connection.setStream(stream);
                    connection.send(b"Hello from server\n");
                    let token = Token(connection.getfd() as usize); // Retrieve the fd before the mutable borrow
                    if let Some(stream) = connection.getStream() {
                        self.poll.registry().register(stream, token, Interest::READABLE);
                    }
                    self.waitingConnections.push(connection);
                }
                Err(e) => {
                    eprintln!("Failed to accept connection: {}", e);
                }
            }
        }
    }

    pub fn getWaitingConnections(&self) -> &Vec<Connection> {
        &self.waitingConnections
    }

    pub fn waitEvent(&mut self) {
        loop {
            self.poll.poll(&mut self.events, None).unwrap();

            let mut tokens_to_handle = vec![];
            for event in &self.events {
                tokens_to_handle.push(event.token());
            }

            for token in tokens_to_handle {
                if token == Token(0) {
                    println!("New connection event");
                    self.handleNewConnection();
                } else {
                    for connection in &mut self.waitingConnections {
                        if token == Token(connection.getfd() as usize) {
                            println!("Connection event");
                        }
                    }
                }
            }
        }
    }
}

