#[cfg(test)]
macro_rules! ipv4 {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        (($a << 24) | ($b << 16) | ($c << 8) | $d) as u32
    }
}

mod utils;
mod ipv4;
mod ipv4mask;
mod subnet;
mod error;

pub use self::ipv4::Ipv4;
pub use self::ipv4mask::Ipv4Mask;
pub use self::subnet::Subnet;
pub use self::error::IpError;

pub use self::utils::*;

