use std::str;
use std::fmt;
use std::ops;
use super::error::IpError;
use super::ipv4::Ipv4;

#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub struct Ipv4Mask(pub u32);

#[allow(dead_code)]
impl Ipv4Mask {
    pub fn from_cidr(cidr: u8) -> Result<Ipv4Mask, IpError> {
        match cidr {
            0...32 => Ok(Ipv4Mask(!(0xffffffff_u64 >> cidr) as u32)),
            _ => Err(IpError::InvalidCIDRValue)
        }
    }
}

impl ops::Deref for Ipv4Mask {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Ipv4Mask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Ipv4::from(self.clone()).fmt(f)
    }
}

impl str::FromStr for Ipv4Mask {
    type Err = IpError;

    fn from_str(s: &str) -> Result<Ipv4Mask, Self::Err> {
        let Ipv4(inner) = try!(s.parse::<Ipv4>());

        let cidr = {
            let mut inner = inner;
            let mut cidr = 32_u32;
            while inner & 0x01 == 0 {
                inner >>= 1;
                cidr -= 1
            }
            !(0xffffffff_u64 >> cidr) as u32
        };

        if inner == cidr {
            Ok(Ipv4Mask(inner))
        }
        else {
            Err(IpError::ParseMaskError)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::{
        Ipv4Mask,
    };

    #[test]
    fn test_parse_invalid_subnet_mask() {
        assert!("255.0.255.0".parse::<Ipv4Mask>().is_err());
        assert!("192.255.168.0".parse::<Ipv4Mask>().is_err());
    }

    #[test]
    fn test_parse_valid_subnet_mask() {
        assert!("255.255.0.0".parse::<Ipv4Mask>().is_ok());
        assert!("255.255.255.255".parse::<Ipv4Mask>().is_ok());
    }

    #[test]
    fn test_from_valid_cidr_value() {
        assert!(Ipv4Mask::from_cidr(32).is_ok());
        assert_eq!(format!("{}", Ipv4Mask::from_cidr(24).unwrap()), "255.255.255.0");
        assert!(Ipv4Mask::from_cidr(24).is_ok());
        assert!(Ipv4Mask::from_cidr(16).is_ok());
        assert!(Ipv4Mask::from_cidr(0).is_ok());
    }

    #[test]
    fn test_from_invalid_cidr_value() {
        assert!(Ipv4Mask::from_cidr(33).is_err());
        assert!(Ipv4Mask::from_cidr(47).is_err());
    }
}

