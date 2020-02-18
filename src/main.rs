mod server;

use server::Server;

fn main() {
    println!("Hello, world!");

    let mut server = Server::new().unwrap();

    loop {
        server.wait_for_event();
    }
}
