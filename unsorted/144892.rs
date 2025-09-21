#![feature(explicit_tail_calls)]
#![allow(incomplete_features)]

fn foo<'a>() -> fn(&'static i32) {
    become bar();
}

fn bar() -> for<'a> fn(&'a i32) {
    dummy
}

fn dummy(_: &i32) {}
