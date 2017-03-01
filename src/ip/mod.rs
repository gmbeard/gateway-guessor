#[cfg(test)]
macro_rules! ipv4 {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        (($a << 24) | ($b << 16) | ($c << 8) | $d) as u32
    }
}

mod utils;
mod ipv4;

pub use self::utils::*;

