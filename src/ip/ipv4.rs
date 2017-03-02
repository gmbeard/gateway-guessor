extern crate std;

use std::str::FromStr;
use std::fmt;
use std::ops;
use std::convert;
use super::error::IpError;
use super::ipv4mask::*;

#[derive(Clone, Copy, PartialEq)] 
#[cfg_attr(test, derive(Debug))]
pub struct Ipv4(pub u32);

impl Ipv4 {
    fn octets(&self) -> [u8; 4] {
        let mut o = [0x00; 4];
        o[0] = ((self.0 & 0xff000000) >> 24)  as u8;
        o[1] = ((self.0 & 0xff0000  ) >> 16)  as u8;
        o[2] = ((self.0 & 0xff00    ) >> 8)   as u8;
        o[3] = ( self.0 & 0xff      )         as u8;
        o
    }
}

impl From<Ipv4Mask> for Ipv4 {
    fn from(mask: Ipv4Mask) -> Ipv4 {
        let Ipv4Mask(inner) = mask;
        Ipv4(inner)
    }
}

impl ops::Deref for Ipv4 {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        return &self.0
    }
}

impl FromStr for Ipv4 {
  type Err = IpError;

  fn from_str(s: &str) -> Result<Ipv4, Self::Err> {
    if s.split(".").count() != 4 {
        return Err(IpError::InvalidOctetCountError);
    }

    let inner = try!( 
       s.split(".")
        .enumerate()
        .fold(Ok(0_u32) as Result<u32, IpError>, |acc, (i, octet)| {
            acc.and_then(|mask| {
                let ibyte = try!(octet.parse::<u8>()) as u32;
                Ok(mask | ibyte << (8 * (std::mem::size_of_val(&mask) - 1 - i)))
            })
        })
    );

    Ok(Ipv4(inner))
  }
}

impl convert::From<u32> for Ipv4 {
    fn from(val: u32) -> Ipv4 {
        Ipv4(val)
    }
}

impl fmt::Display for Ipv4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let octets = self.octets();
        write!(f, "{}.{}.{}.{}", octets[0], octets[1], 
            octets[2], octets[3])
    }
}

impl ops::Not for Ipv4 {
    type Output = Ipv4;

    fn not(self) -> Self::Output {
        let Ipv4(inner) = self;
        Ipv4(!inner)
    }
}

impl ops::BitAnd for Ipv4 {
    type Output = Ipv4;
    fn bitand(self, rhs: Self) -> Self::Output {
        let Ipv4(lhs) = self;
        let Ipv4(rhs) = rhs;
        Ipv4(lhs & rhs)
    }
}

impl ops::BitAnd<Ipv4Mask> for Ipv4 {
    type Output = Ipv4;
    fn bitand(self, rhs: Ipv4Mask) -> Self::Output {
        self & Ipv4::from(rhs)
    }
}

impl ops::BitOr for Ipv4 {
    type Output = Ipv4;
    fn bitor(self, rhs: Self) -> Self::Output {
        let Ipv4(lhs) = self;
        let Ipv4(rhs) = rhs;
        Ipv4(lhs | rhs)
    }
}

impl ops::BitOr<Ipv4Mask> for Ipv4 {
    type Output = Ipv4;
    fn bitor(self, rhs: Ipv4Mask) -> Self::Output {
        self | Ipv4::from(rhs)
    }
}

impl ops::Sub for Ipv4 {
    type Output = Ipv4;
    fn sub(self, rhs: Self) -> Self::Output {
        let Ipv4(lhs) = self;
        let Ipv4(rhs) = rhs;
        Ipv4(lhs - rhs)
    }
}

impl ops::Add for Ipv4 {
    type Output = Ipv4;
    fn add(self, rhs: Self) -> Self::Output {
        let Ipv4(lhs) = self;
        let Ipv4(rhs) = rhs;
        Ipv4(lhs + rhs)
    }
}

impl ops::Sub<u32> for Ipv4 {
    type Output = Ipv4;
    fn sub(self, rhs: u32) -> Self::Output {
        let Ipv4(inner) = self;
        Ipv4(inner - rhs)
    }
}

impl ops::Add<u32> for Ipv4 {
    type Output = Ipv4;
    fn add(self, rhs: u32) -> Self::Output {
        let Ipv4(inner) = self;
        Ipv4(inner + rhs)
    }
}

#[cfg(test)]
mod tests {

    use super::{
      Ipv4,
    };

    #[test]
    fn test_parse() {
      let result = "192.168.0.1".parse::<Ipv4>();

      assert!(result.is_ok());

      let Ipv4(inner) = result.unwrap();
      assert_eq!(inner, ipv4!(192,168,0,1));
    }

    #[test]
    fn test_parse_invalid() {
        assert!("192.168.0.2.4.5".parse::<Ipv4>().is_err());
        assert!("192.168".parse::<Ipv4>().is_err());
    }

    #[test]
    fn test_fmt() {
        let result = "192.168.0.254".parse::<Ipv4>().unwrap();
        assert_eq!(format!("{}", result), "192.168.0.254");
    }

    #[test]
    fn test_not() {
        let result = "255.255.255.0".parse::<Ipv4>().unwrap();
        assert_eq!(!result, Ipv4(ipv4!(0,0,0,255)));
    }

    #[test]
    fn test_bitor() {
        let result = "255.255.0.0".parse::<Ipv4>().unwrap();
        assert_eq!(result | !result, Ipv4(ipv4!(255,255,255,255)))
    }

    #[test]
    fn test_bitand() {
        let ip = "10.11.12.1".parse::<Ipv4>().unwrap();
        let mask = "255.252.0.0".parse::<Ipv4>().unwrap();
        assert_eq!(ip & mask, Ipv4(ipv4!(10,8,0,0)))
    }

    #[test]
    fn test_add_and_subtract() {
        let result = "192.168.0.1".parse::<Ipv4>().unwrap();
        assert_eq!(result + 4, Ipv4(ipv4!(192,168,0,5)));
        assert_eq!(result - 1, Ipv4(ipv4!(192,168,0,0)));
    }
}
