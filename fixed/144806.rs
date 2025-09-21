#![feature(explicit_tail_calls)]
#![allow(incomplete_features)]

fn foo(_: ()) {
    unsafe {
        become std::mem::transmute::<(), ()>(());
    }
}

fn main() {
    foo(());
}
