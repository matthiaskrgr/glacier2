#![feature(effects)]

const fn foo(n: i32) -> i32 {
    n
}

const fn return_ty_mismatch() {
    const_eval_select((1,), foo, bar);
}
