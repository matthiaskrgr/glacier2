// ignore-wasm32 compiled with panic=abort by default
// unit-test: CopyProp

#![feature(custom_mir, core_intrinsics)]
#![allow(unused_assignments)]
extern crate core;
use core::intrinsics::mir::*;

struct NotCopy(bool);

// EMIT_MIR custom_move_arg.f.CopyProp.diff
#[custom_mir(dialect = "analysis", phase = "post-cleanup")]
fn f(_1: NotCopy) {}

#[inline(never)]
fn opaque<T>(_t: T) {}

fn main() {
    f(NotCopy(true));
    println!("hi");
}
