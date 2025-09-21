#![feature(explicit_tail_calls)]
#![allow(incomplete_features)]

fn main() {
    let _ = foo(async {});
}

fn foo<T>(x: T) -> T {
    become bar(x);
}

fn bar<T>(x: T) -> T {
    x
}
