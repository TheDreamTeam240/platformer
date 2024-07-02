mod connection;
mod server;

use crate::server::Server;

fn main() {
    let mut server : server::Server = Server::new();
    server.init(8080);
    server.run();
    while server.isRunning() {
        server.waitEvent();
    }
}
