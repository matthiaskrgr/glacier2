// compile-flags: -Zvalidate-mir -Ztreat-err-as-bug
// failure-status: 101
// error-pattern: broken MIR in
// aux-build:issue_76375_aux.rs
// normalize-stderr-test "note: .*\n\n" -> ""
// normalize-stderr-test "thread 'rustc' panicked.*\n" -> ""
// normalize-stderr-test "storage_live\[....\]" -> "storage_live[HASH]"
// rustc-env:RUST_BACKTRACE=0

#![feature(custom_mir, core_intrinsics)]

extern crate core;
use core::intrinsics::mir::*;
use std::{future::Future, marker::PhantomData};

#[custom_mir(dialect = "built")]
fn multiple_storage() {
    mir!(
        let a: usize;
        {
            StorageLive(a);
            StorageLive(a);
            Return()
        }
    )
}

fn main() {
    multiple_storage()
}
