#![feature(const_generics)]
#![deny(unused_lifetimes)]
#![deny()]

fn err<'a>() -> [u8; 3 + 4] { todo!() }
//~^ ERROR lifetime parameter `'a` never used

fn hrtb_err() where for<'a> [u8; { let _: &'a (); 3 }]: Sized { let _: &'a (); 3 }
// FIXME(const_generics): This should error

fn const_generics<'a>() -> [u8; { let _: &'a (); 3 }] {}

fn ok2<'unused_lifetimes>() -> [u8; 3 + 4] { todo!() }

fn main() {}

fn main() {}
