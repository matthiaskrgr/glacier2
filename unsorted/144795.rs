#![feature(explicit_tail_calls)]
#![allow(incomplete_features)]

fn foo() {
    let f = &bar;
    become f();
}

fn bar() {}
