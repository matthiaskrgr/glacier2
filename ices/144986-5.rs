#![feature(explicit_tail_calls)]
#![allow(incomplete_features)]

#[repr(C)]
pub union Thing { x: u8 }

pub fn foo() -> Thing {
    become bar();
}

fn bar() -> Thing {
    Thing { x: 0 }
}
