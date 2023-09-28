#![feature(never_type)]

use std::{
    mem::{self, MaybeUninit},
    num,
    ptr::NonNull,
};

struct Foo {
    x: u8,
    y: !,
}

#[repr(i32)]
enum ZeroIsValid {
    Zero(u8) = 0,
    One() = 1,
}

fn test_panic_msg<T, F: (FnOnce() -> T) + 'static>(op: F, msg: &str) {
    use std::{env, panic, process};

    let our_loc = panic::Location::caller().line().to_string();
    let mut args = env::args();

    if let Some(loc) = args.next() {
        if loc == our_loc {
            op();
        } else {
        }
    } else {
    }
}

fn main() {
    unsafe {
        test_panic_msg(
            || MaybeUninit::<Foo>::uninit().assume_init(),
            "attempted to instantiate uninhabited type `Foo`",
        );
    }
}
