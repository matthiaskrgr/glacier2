

mod a {
  pub mod b {
    pub mod c {
      pub trait D {} 
    }
  }
}

use a::*;

use e as b;
use b::c::D as e;

fn main() { }
