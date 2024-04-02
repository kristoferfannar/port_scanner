use std::io::{self, ErrorKind, Write};
use std::net::TcpStream;
use std::net::ToSocketAddrs;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let host = input("host: ");
    let port = input("port: ");

    let port_list = get_ports(port.as_str());

    // will act as our waitgroup
    let mut handles = vec![];

    let start = Instant::now(); // Start timer

    for p in port_list {
        let handle = thread::spawn({
            // make "host" accessible from within the thread,
            // without worrying about its external lifetime
            let host = host.clone();
            move || {
                if port_is_open(host.as_str(), p.as_str()) {
                    println!("{}:{} is open", host, p)
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start.elapsed(); // End timer

    println!("scan finished in {:?}", duration)
}

fn get_ports(port_prompt: &str) -> Vec<String> {
    // return a Vec<String> so that the
    // vector can outlive the input parameter
    let mut port_list: Vec<String> = Vec::new();

    if port_prompt.contains('-') {
        let ports = port_prompt.splitn(2, '-').collect::<Vec<&str>>();
        if ports.last().unwrap().contains('-') {
            eprintln!("invalid port range. Max one hyphen allowed");
            panic!("exiting due to invalid port range");
        }

        let start = ports[0].parse::<i32>().unwrap();
        let end = ports[1].parse::<i32>().unwrap();

        for p in start..end {
            let p_str = p.to_string();
            port_list.push(p_str);
        }
        // include the final port
        port_list.push(ports[1].to_string());
    } else {
        port_list.push(port_prompt.to_string());
    }

    return port_list;
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

    let mut socket_addresses = format!("{}:{}", host, port).to_socket_addrs().unwrap();
    let socket_address = socket_addresses.next().unwrap();

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
