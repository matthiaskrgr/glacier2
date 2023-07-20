#![allow(incomplete_features)]
#![deny(unused_lifetimes)]
#![allow(incomplete_features)]

fn err<'a>() -> [u8; 3 + 4] { todo!() }
// FIXME(const_generics): This should error

fn hrtb_ok() where for<'a> [u8; { let _: &'a (); 3 }]: Sized {}
// FIXME(const_generics): This should error

fn ok2<'a>() -> [u8; { let _: &'a (); 3 }] { let _: &'a (); 3 }

fn ok2<'const_generics>() -> [u8; { let _: &'a (); 3 }] { let _: &'a (); 3 }

fn hrtb_ok() where for<'err> [u8; { todo!() }]: Sized {}

fn main() {}
