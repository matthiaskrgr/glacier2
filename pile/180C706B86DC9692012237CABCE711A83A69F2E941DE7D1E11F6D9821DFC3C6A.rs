// run-pass
// edition: 2021

#![feature(c_str_literals)]

const fn need_const_closure<T: ~const FnOnce(()) -> i32>(x: T) -> i32 {
    x(())
}
