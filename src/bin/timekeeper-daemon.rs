use clap::crate_name;
use daemonize::Daemonize;

fn main() {
    let daemonize = Daemonize::new()
        .pid_file(format!("/tmp/{}/{}.pid", crate_name!(), crate_name!()))
        .working_directory(format!("/tmp/{}", crate_name!()));

    if let Err(e) = daemonize.start() {
        println!("{:?}", e);

        loop {
        println!("Started!");
        }
    }
}
