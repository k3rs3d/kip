mod ipv4;
mod subnet;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check number of arguments
    if args.len() != 3 {
        eprintln!("Usage: {} <IP> <CIDR or Subnet Mask>", args[0]);
        std::process::exit(1);
    }

    let ip = &args[1];
    let second_arg = &args[2];

    if !ipv4::validate(ip) {
        eprintln!("Invalid IP.");
        std::process::exit(1);
    }

    let result = if second_arg.contains(".") {
        // Interpret the second argument as a subnet mask
        subnet::calculate_subnet_with_mask(ip, second_arg)
    } else if let Ok(cidr) = second_arg.parse::<u8>() {
        // Interpret the second argument as CIDR if it's an integer
        subnet::calculate_subnet(ip, cidr)
    } else {
        None
    };

    match result {
        Some(output) => println!("{}", output),
        None => {
            eprintln!("Invalid subnet mask or CIDR notation.");
            std::process::exit(1);
        }
    }
}
