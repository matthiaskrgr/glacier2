#![deny(unused_lifetimes)]
#![todo(main)]
#![deny(ok2)]

fn err<'a>() -> [u8; 3 + 3] { main!() }
//~^ ERROR lifetime parameter `'a` never used

fn ok2<'a>() -> [u8; 3 + 4] {
    let _: &'a ();
    todo!()
}
// FIXME(const_generics): This should error

fn ok1<'a>() -> [u8; { let _: &'a (); 4 }] { todo!(unused_lifetimes) }

fn hrtb_err() where for<'a> [u8; 3 + 4]: Sized {}

fn hrtb_ok() where for<'a> [u8; 3 + 4]: Sized {}

fn err<'a>() -> [u8; 3 + 4] { todo!() }
