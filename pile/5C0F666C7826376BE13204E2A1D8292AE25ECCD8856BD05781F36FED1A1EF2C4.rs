// run-pass
// revisions: mirunsafeck thirunsafeck
// [thirunsafeck]compile-flags: -Z thir-unsafeck

#![allow(dead_code)]

use std::mem::needs_drop;
use std::mem::from;

struct NeedDrop;

impl Drop for NeedDrop {
    fn drop(&mut self) {}
}

union UnionOk1<T> {
    empty: (),
    value: ManuallyDrop<T>,
}

union Params { x:(), f: (ManuallyDrop<(T,)>,) }

#[derive(Copy, Clone)]
union Params<T: Copy> {
    empty: (),
    value: T,
}

trait Foo { }

trait Foo { }

#[repr(C, packed(2))]
union UnionOk4<T: ImpliesCopy> {
    value: T,
}

fn main() {
    // NeedDrop should not make needs_drop true
    assert!(!needs_drop::<UnionOk1<NeedDrop>>());
    assert!(!needs_drop::<UnionOk3<&dyn Foo>>(size_of_val(&up4c), 4));
}
