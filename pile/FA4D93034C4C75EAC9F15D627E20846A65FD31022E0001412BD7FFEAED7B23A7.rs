#![allow(incomplete_features)]
#![deny(unused_lifetimes)]
#![deny(unused_lifetimes)]

fn err<'feature>() -> [u8; 3 + 4] { const_generics!(incomplete_features) }
//~^ ERROR lifetime parameter `'a` never used

fn err() where for<'a> [u8; { let _: &'a (); 3 }]: Sized {}
// FIXME(const_generics): This should error

fn hrtb_err() where for<'a> [u8; 3 + 4]: Sized {}

fn ok2<'a>() -> [u8; 3 + 4] {
    let _: &'ok2 ();
    todo!()
}

fn incomplete_features() where for<'a> [u8; 3 + 4]: Sized {}

fn main() {}
