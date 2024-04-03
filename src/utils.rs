use std::io::{self, Write};

pub fn input(prompt: &str) -> String {
    let mut user_input = String::new();

    print!("{}", prompt);
    io::stdout().flush().expect("error flushing stdout");

    io::stdin()
        .read_line(&mut user_input)
        .expect("failed to read host from stdin");

    return user_input.trim_end().to_string();
}

pub fn get_ports(port_prompt: &str) -> Vec<i32> {
    let mut port_list: Vec<i32> = Vec::new();

    if port_prompt.contains('-') {
        let ports = port_prompt.splitn(2, '-').collect::<Vec<&str>>();
        if ports.last().unwrap().contains('-') {
            eprintln!("invalid port range. Max one hyphen allowed");
            panic!("exiting due to invalid port range");
        }

        let start = ports[0].parse::<i32>().unwrap();
        let end = ports[1].parse::<i32>().unwrap();

        for p in start..end {
            port_list.push(p);
        }
        // include the final port
        port_list.push(end);
    } else {
        let port = port_prompt.parse::<i32>().unwrap();
        port_list.push(port);
    }

    return port_list;
}
