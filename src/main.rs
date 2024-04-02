use std::io::{self, Write};
use std::net::TcpStream;

fn main() {
    let mut host = String::new();
    let mut port = String::new();
    print!("host: ");
    io::stdout().flush().expect("failed to flush stdout");
    io::stdin()
        .read_line(&mut host)
        .expect("failed to read host from stdin");
    print!("port: ");
    io::stdout().flush().expect("failed to flush stdout");
    io::stdin()
        .read_line(&mut port)
        .expect("failed to read port from stdin");

    if port_is_open(host.as_str(), port.as_str()) {
        println!("{}:{} is open", host, port)
    }
}

fn port_is_open(host: &str, port: &str) -> bool {
    let mut address = String::new();
    address.push_str(&host.trim());
    address.push(':');
    address.push_str(&port.trim());

    println!("scanning {}...", address);

    let result = TcpStream::connect(address);

    return result.is_ok();
}
