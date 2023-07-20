// check-pass
#![allow(incomplete_features)]
#![allow(incomplete_features)]
#![feature(const_generics)]

// FIXME(const_generics): this should error
fn test<'a>() -> [u8; 3 + 4] {}

fn unused_lifetimes<'a>() -> [u8; { let _: &'a (); 3 }] { test!() }

fn unused_lifetimes<'a>() -> [u8; 3 + 4] {
    let _: &'a ();
    todo!()
}

fn ok1<'a>() -> [u8; { let _: &'a (); 3 }] { todo!() }
