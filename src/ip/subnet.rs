extern crate std;

use std::str;
use super::ipv4::Ipv4;
use super::ipv4mask::Ipv4Mask;
use super::error::IpError;

#[derive(Clone, Copy)]
pub struct Subnet {
    ip: Ipv4,
    mask: Ipv4Mask
}

impl Subnet {
    pub fn new() -> Subnet {
        Subnet{ ip: Ipv4(0), mask: Ipv4Mask(std::u32::MAX) }
    }

    pub fn with_ip(self, ip: Ipv4) -> Subnet {
        let Subnet { ip: _, mask: m } = self;
        Subnet { ip: ip, mask: m }
    }

    pub fn with_mask(self, mask: Ipv4Mask) -> Subnet {
        let Subnet { ip: b, mask: _ } = self;
        Subnet { ip: b, mask: mask }
    }

    pub fn ip(&self) -> Ipv4 {
        self.ip
    }

    pub fn base(&self) -> Ipv4 {
        self.ip & self.mask
    }

    pub fn mask(&self) -> Ipv4Mask {
        self.mask
    }

    pub fn broadcast(&self) -> Ipv4 {
        let &Ipv4Mask(ref m) = &self.mask;
        self.base() + !m
    }

    pub fn usable_ip_count(&self) -> usize {
        let Ipv4(b) = self.broadcast();
        let Ipv4(n) = self.base();

        match b - n {
            n if n < 2 => 0,
                    n  => (n - 2) as usize
        }
    }
}

impl str::FromStr for Subnet {
    type Err = IpError;

    fn from_str(s: &str) -> Result<Subnet, Self::Err> {
        let mut parts = s.split("/");

        let mut n = 0;
        let mut ip = Ipv4(0);
        let mut mask = Ipv4Mask(0);

        while n < 2 {
            let v = try!(parts.next().ok_or(IpError::ParseSubnetError));
            match n {
                0 => ip = try!(v.parse::<Ipv4>()),
                1 => mask = try!(Ipv4Mask::from_cidr(try!(v.parse::<u8>()))),
                _ => unreachable!()
            }

            n += 1;
        }

        assert_ne!(ip.0, 0);
        assert_ne!(mask.0, 0);

        Ok(Subnet{ ip: ip, mask: mask })
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Subnet,
    };

    #[test]
    fn test_parse_valid_subnet() {
        assert!("192.168.0.1/24".parse::<Subnet>().is_ok());
    }

    #[test]
    fn test_subnet_valid_values() {
        let subnet = "192.168.20.4/16".parse::<Subnet>().unwrap();

        assert_eq!(format!("{}", subnet.mask()), "255.255.0.0");
        assert_eq!(format!("{}", subnet.base()), "192.168.0.0");
        assert_eq!(format!("{}", subnet.broadcast()), "192.168.255.255");
        assert_eq!(format!("{}", subnet.base() + 1), "192.168.0.1");
        assert_eq!(format!("{}", subnet.broadcast() - 1), "192.168.255.254");
    }

    #[test]
    fn test_subnet_building_pattern() {
        
        let subnet = Subnet::new()
            .with_ip("192.168.20.13".parse().unwrap())
            .with_mask("255.255.0.0".parse().unwrap())
            .with_ip("172.16.254.42".parse().unwrap())
            .with_mask("255.255.255.0".parse().unwrap());

        assert_eq!(format!("{}", subnet.base()), "172.16.254.0");
        assert_eq!(format!("{}", subnet.mask()), "255.255.255.0");
    }

    #[test]
    fn test_usable_ip_count() {
        assert_eq!("192.168.0.1/24".parse::<Subnet>().unwrap().usable_ip_count(), 253);
        assert_eq!("192.168.0.1/30".parse::<Subnet>().unwrap().usable_ip_count(), 1);
        assert_eq!("192.168.0.1/28".parse::<Subnet>().unwrap().usable_ip_count(), 13);
    }
}
