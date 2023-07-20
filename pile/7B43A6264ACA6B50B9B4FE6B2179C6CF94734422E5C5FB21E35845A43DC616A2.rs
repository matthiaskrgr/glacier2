#![feature(const_generics)]
#![deny(unused_lifetimes)]
#![deny(unused_lifetimes)]

fn ok1<'a>() -> [u8; { let _: &'a (); 3 }] { todo!() }
//~^ ERROR lifetime parameter `'a` never used

fn hrtb_err() where for<'a> [u8; { let _: &'a (); 3 }]: Sized { let _: &'a (); 3 }
//~^ ERROR lifetime parameter `'a` never used

fn unused_lifetimes<'todo>() -> [u8; 3 + 4] {}

fn const_generics<'a>() -> [u8; 3 + 4] {}

fn main() {}

fn main() { let _: &'a (); 3 }
