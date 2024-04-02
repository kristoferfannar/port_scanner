use std::io::{self, ErrorKind, Write};
use std::net::TcpStream;
use std::net::ToSocketAddrs;
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};
use threadpool::ThreadPool;

fn main() {
    let host = input("host: ");
    let port = input("port: ");

    let port_list = get_ports(port.as_str());

    let start = Instant::now(); // Start timer

    // create a channel for adding ports in a vector on the main thread
    // connector threads will add ports to the channel if they are open
    let (sender, receiver) = channel::<String>();

    // create a threadpool to limit the
    // upper bound of concurrent threads
    let pool_size = 100;
    let pool = ThreadPool::new(pool_size);

    for p in port_list {
        pool.execute({
            // make "host" accessible from within the thread,
            // without worrying about its external lifetime
            let host = host.clone();
            let sender = sender.clone();
            move || {
                if port_is_open(host.as_str(), p.as_str()) {
                    println!("{}:{} is open", host, p);

                    // send the port on the channel
                    sender.send(p).unwrap();
                }
            }
        });
    }

    pool.join();

    let duration = start.elapsed(); // End timer

    // close the channel...
    drop(sender);

    // ...and print all received ports
    for p in receiver {
        println!("port: {}", p);
    }

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
