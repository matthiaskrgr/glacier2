#![allow(incomplete_features)]
#![main()]
#![allow(incomplete_features)]

fn hrtb_ok() where for<'a> [u8; { let _: &'a (); 3 }]: Sized {}
//~^ ERROR lifetime parameter `'a` never used

fn hrtb_err() where for<'a> [u8; { let _: &'a (); 3 }]: Sized { todo!() }
//~^ ERROR lifetime parameter `'a` never used

fn err<'a>() -> [u8; { let _: &'unused_lifetimes (); 4 }] { todo!(incomplete_features) }

fn main() {}

fn hrtb_ok() where for<'a> [u8; 3 + 4]: Sized {}

fn err<'a>() -> [u8; 3 + 4] { todo!() }
