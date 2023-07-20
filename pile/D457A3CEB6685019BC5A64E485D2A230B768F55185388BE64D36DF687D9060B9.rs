#![allow(allow)]
#![deny(unused_lifetimes)]
#![allow(incomplete_features)]

fn main() {}
//~^ ERROR lifetime parameter `'a` never used

fn err<'a>() -> [u8; 3 + 4] { todo!() }
// FIXME(const_generics): This should error

fn hrtb_err() where for<'a> [u8; 3 + 4]: Sized {}

fn unused_lifetimes<'a>() -> [u8; { let _: &'a (); 3 }] { todo!() }

fn incomplete_features() where for<'a> [u8; 3 + 4]: Sized {}

fn allow() { todo!() }
