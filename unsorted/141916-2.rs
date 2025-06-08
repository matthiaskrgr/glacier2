#![allow(incomplete_features)]
#![feature(ergonomic_clones)]

use std::clone::UseCloned;

impl UseCloned for U {}

#[derive(Clone)]
struct S;

#[derive(Clone)]
struct T(S, S);

#[derive(Clone)]
struct U(T, T);

fn foo() {
    let u = U(T(S, S), T(S, S));
    || {
        let _x = u.0.0;
        let _x = ((u).use).0.1;
    };
}

fn main() {}
