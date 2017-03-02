extern crate std;

use std::num::ParseIntError;

use super::ipv4::Ipv4;
use super::ipv4mask::Ipv4Mask;
use super::error::IpError;

#[derive(Debug, PartialEq)]
pub struct NetworkAddr<T>(pub T);

#[derive(Debug, PartialEq)]
pub struct BroadcastAddr<T>(pub T);

type NetworkValues = (NetworkAddr<u32>, BroadcastAddr<u32>);

pub fn is_valid(address: &str) -> bool {
    let octets = address.split(".");
    octets.count() == 4 && make_mask_from_string(address).is_ok()
}

pub fn make_mask_from_cidr(cidr: u8) -> Result<u32, ()> {
    if cidr > 32 {
        return Err(());
    }
    let mask = 0xffffffff_u32 & !((0xffffffff_u64 >> cidr) as u32);   
    Ok(mask)
}

pub fn make_mask_from_string(address: &str) -> Result<u32, ParseIntError> {
    address.split(".")
        .enumerate()
        .fold(Ok(0_u32), |acc, (i, octet)| {
            acc.and_then(|mask| {
                let ibyte = try!(octet.parse::<u8>()) as u32;
                Ok(mask | ibyte << (8 * (std::mem::size_of_val(&mask) - 1 - i)))
            })
        })
}

pub fn make_ip_from_mask(mask: u32) -> String {
    let oct1 = ((mask & 0xFF000000) >> 24)  as u8;
    let oct2 = ((mask & 0xFF0000  ) >> 16)  as u8;
    let oct3 = ((mask & 0xFF00    ) >> 8)   as u8;
    let oct4 = ( mask & 0xFF      )         as u8;

    format!("{}.{}.{}.{}", oct1, oct2, oct3, oct4)
}

pub fn calc_network_values(host: &str, mask: u32) -> Result<NetworkValues, IpError> {
    let ip = try!(host.parse::<Ipv4>());
    let mask = Ipv4Mask(mask); 

    let Ipv4(net) = ip & mask;
    let Ipv4(bcast) = ip | !Ipv4::from(mask);

    Ok((NetworkAddr(net), BroadcastAddr(bcast)))
}

#[cfg(test)]
pub mod tests {
    use super::{
        make_ip_from_mask,
        make_mask_from_string,
        make_mask_from_cidr,
        calc_network_values,
        is_valid,
        NetworkAddr,
        BroadcastAddr
    };

    #[test]
    fn test_ip_from_mask() {
        let mask = ipv4!(192,168,255,255);
        assert_eq!("192.168.255.255", make_ip_from_mask(mask));
    }

    #[test]
    fn test_mask_from_string() {
        let mask = ipv4!(192,168,16,1);
        assert_eq!(mask, make_mask_from_string("192.168.16.1").unwrap());
    }

    #[test]
    fn test_invalid_ipv4_is_invalid() {
        assert!(!is_valid("500.168.16.1"));
    }

    #[test]
    fn test_invalid_ipv4_from_string() {
        assert!(make_mask_from_string("500.168.16.1").is_err());
    }

    #[test]
    fn test_network_values() {
        let network = ipv4!(192,168,0,0);
        let broadcast = ipv4!(192,168,255,255);
        let mask = make_mask_from_cidr(16).unwrap();

        assert_eq!((NetworkAddr(network), BroadcastAddr(broadcast)), 
            calc_network_values("192.168.0.15", mask).unwrap());
    }
}
