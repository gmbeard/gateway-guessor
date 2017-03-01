extern crate std;

use std::str::FromStr;

struct Ipv4(u32);

impl FromStr for Ipv4 {
  type Err = ();

  fn from_str(s: &str) -> Result<Ipv4, Self::Err> {
    use super::utils::make_mask_from_string;
    let inner = try!(make_mask_from_string(s).map_err(|_| ()));
    Ok(Ipv4(inner))
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
}
