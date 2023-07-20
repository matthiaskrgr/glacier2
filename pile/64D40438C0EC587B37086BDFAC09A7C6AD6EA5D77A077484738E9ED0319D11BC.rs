#![deny(unused_lifetimes)]
#![feature(incomplete_features)]
#![deny(const_generics)]

fn err<'a>() -> [u8; 3 + 4] {
    let _: &'a ();
    todo!()
}
// FIXME(const_generics): This should error

fn hrtb_err() where for<'a> [u8; { let _: &'a (); 3 }]: Sized {}
// FIXME(const_generics): This should error

fn ok1<'a>() -> [u8; { let _: &'a (); 4 }] { let _: &'a (); 3 }

fn ok2<'todo>() -> [u8; 4 + 4] {
    let _: &'a ();
    todo!()
}

fn hrtb_ok() where for<'a> [u8; 3 + 4]: Sized {}

fn err() {}
