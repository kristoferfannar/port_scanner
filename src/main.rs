use std::time::Instant;
mod ports;
mod utils;

fn main() {
    let host = utils::input("host: ");
    let port = utils::input("port: ");
    let port_list = utils::get_ports(port.as_str());

    let start = Instant::now(); // Start timer
    let open_ports = ports::find_open_ports(host.as_str(), port_list);
    let duration = start.elapsed(); // End timer

    for p in open_ports {
        println!("Open port: {}", p);
    }

    println!("scan finished in {:?}", duration)
}
