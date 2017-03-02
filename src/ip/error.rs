extern crate std;

use std::num::ParseIntError;
use std::convert::From;

#[allow(dead_code)]
#[cfg_attr(test, derive(Debug))]
pub enum IpError {
    ParseMaskError,
    ParseIpv4Error(ParseIntError),
    InvalidOctetCountError,
    ParseSubnetError,
    InvalidCIDRValue
}

impl From<ParseIntError> for IpError {
    fn from(e: ParseIntError) -> IpError {
        IpError::ParseIpv4Error(e)
    }
}

