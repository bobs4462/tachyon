use std::env;
use std::net::SocketAddr;
use std::process;

pub fn address_get() -> SocketAddr {
    let mut args = env::args();
    let executable = args.next().unwrap();
    match args.next() {
        Some(addr) => match addr.parse() {
            Ok(socket) => socket,
            Err(e) => {
                eprintln!("Error obtaining address: {}", e);
                process::exit(1);
            }
        },
        None => {
            eprintln!(
                "No arguments provided.\n\
                 Usage: {0} {{ip_address:port}}\n\
                 Example: {0} 127.0.0.1:8080",
                executable
            );
            process::exit(1);
        }
    }
}
