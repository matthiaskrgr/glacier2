// OK

#![deny(non_snake_case)]

#[no_mangle]
fn main() {} // check-pass

pub struct Foo;

impl Foo {
    #[no_mangle]
    pub extern "C" fn main() {} // OK
}

fn non_snake_case() {}
