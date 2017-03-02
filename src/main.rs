use std::env;
pub mod ip;
use std::process;

fn convert_to_ip_and_mask(a: &str, b: &str) 
    -> Result<(ip::Ipv4, ip::Ipv4Mask), ip::IpError>
{
    let r = (
        try!(a.parse()),
        try!(b.parse())
    );

    Ok(r)
}

fn guess_gateway(host: &str, subnet_mask: u32) {
    use ip::{
        NetworkAddr,
        BroadcastAddr,
        calc_network_values,
        make_ip_from_mask,
    };

    if let Ok((NetworkAddr(network), BroadcastAddr(broadcast))) = calc_network_values(host, subnet_mask) {
        println!("The network address for this host is: {}", make_ip_from_mask(network));
        println!("The broadcast address for this host is: {}", make_ip_from_mask(broadcast));
        println!("I'm guessing the gateway is {} or {}", make_ip_from_mask(network+1), make_ip_from_mask(broadcast-1));
    }
    else {
        println!("Couldn't parse '{}' as a valid IPv4 address", host);
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            match args[1].parse::<ip::Subnet>() {
                Ok(s) => guess_gateway(&format!("{}", s.ip()), *s.mask()),
                Err(_) => {
                    println!("It looks like your IP/CIDR is invalid: '{}'", args[1]);
                    process::exit(1);
                }
            }
        },
        3 => {
            if let Ok((ip, mask)) = convert_to_ip_and_mask(&args[1], &args[2]) {
                guess_gateway(&format!("{}", ip), *mask);
            }
            else {
                println!("It looks like your IP or Subnet mask is invalid: '{} {}'", args[1], args[2]);
                process::exit(1)
            }
        },
        _ => {
            println!("Gateway Guesser
Returns expected Gateway address for a given IP and subnet

Usage:
    gateway IP/mask CIDR notation or
    gateway IP mask

Examples:
    gateway 192.168.0.2/24
    gateway 192.168.0.2 255.255.255.0")
        }
    }
}
