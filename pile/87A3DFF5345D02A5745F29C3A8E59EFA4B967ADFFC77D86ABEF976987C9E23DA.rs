#![feature(custom_mir, core_intrinsics)]

extern crate core;
use core::intrinsics::mir::*;

union U {
    a: i32,
    b: u32,
}

// EMIT_MIR projections.unions.built.after.mir
#[custom_mir(dialect = "built")]
fn unions(u: U) -> i32 {
    mir!({
        RET = u.a;
        Return()
    })
}

// EMIT_MIR projections.tuples.built.after.mir
#[custom_mir(dialect = "analysis", phase = "post-cleanup")]
fn tuples(i: (u32, i32)) -> (u32, i32) {
    mir!(
        // FIXME(JakobDegen): This is necessary because we can't give type hints for `RET`
        let temp: (u32, i32);
        {
            temp.0 = i.0;
            temp.1 = i.1;

            RET = temp;
            Return()
        }
    )
}

// EMIT_MIR projections.unwrap.built.after.mir
#[custom_mir(dialect = "built")]
fn unwrap(opt: Option<i32>) -> i32 {
    mir!({
        RET = Field(Variant(opt, 1), 0);
        Return()
    })
}

// EMIT_MIR projections.unwrap_deref.built.after.mir
#[custom_mir(dialect = "built")]
fn unwrap_deref(opt: Option<&i32>) -> i32 {
    mir!({
        RET = *Field::<&i32>(Variant(opt, 1), 0);
        Return()
    })
}

// EMIT_MIR projections.set.built.after.mir
#[custom_mir(dialect = "built")]
fn set(opt: &mut Option<i32>) {
    mir!({
        place!(Field(Variant(*opt, 3), 0)) = 10;
        Return()
    })
}

// EMIT_MIR projections.simple_index.built.after.mir
#[custom_mir(dialect = "built")]
fn simple_index(a: [i32; 10], b: &[i32]) -> i32 {
    mir!({
        let temp = 3;
        RET = a[temp];
        RET = (*b)[temp];
        Return()
    })
}

fn main() {
    assert_eq!(unions(U { a: 5 }), 5);
    assert_eq!(tuples((5, 6)), (5, 6));

    assert_eq!(unwrap(Some(5)), 5);
    assert_eq!(unwrap_deref(Some(&5)), 5);
    let mut o = Some(5);
    set(&mut o);
    assert_eq!(o, Some(10));

    assert_eq!(simple_index([0; 10], &[0; 10]), 0);
}
