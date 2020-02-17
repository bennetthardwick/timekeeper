mod server;

use server::Server;
use server::Event;

fn main() {
    println!("Hello, world!");

    let server = Server::new().unwrap();
    server.enable_window_events().unwrap();

    for event in server.events() {
        match event {
            Event::FocusChange => server.snapshot_active_window().unwrap()
        };
        println!("{:?}", event);
    }
}
