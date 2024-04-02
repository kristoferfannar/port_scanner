use std::io::{self, ErrorKind, Write};
use std::net::TcpStream;
use std::net::ToSocketAddrs;
use std::time::Duration;

fn main() {
    let host = input("host: ");
    let port = input("port: ");

    if port_is_open(host.as_str(), port.as_str()) {
        println!("{}:{} is open", host, port)
    } else {
        println!("{}:{} is closed", host, port)
    }
}

fn input(prompt: &str) -> String {
    let mut user_input = String::new();

    print!("{}", prompt);
    io::stdout().flush().expect("error flushing stdout");

    io::stdin()
        .read_line(&mut user_input)
        .expect("failed to read host from stdin");

    return user_input.trim_end().to_string();
}

fn port_is_open(host: &str, port: &str) -> bool {
    let mut address = String::new();
    address.push_str(&host.trim());
    address.push(':');
    address.push_str(&port.trim());

    println!("scanning {}...", address);

    let mut socket_addresses = format!("{}:{}", host, port).to_socket_addrs().unwrap();
    let socket_address = socket_addresses.next().unwrap();

    println!("{}", socket_address.to_string());

    let result = TcpStream::connect_timeout(&socket_address, Duration::from_secs(1));

    if let Err(e) = result {
        match e.kind() {
            ErrorKind::TimedOut => {}
            ErrorKind::ConnectionRefused => {}
            _ => {
                println!("Error: {}", e);
            }
        }
        return false;
    }

    return result.is_ok();
}
