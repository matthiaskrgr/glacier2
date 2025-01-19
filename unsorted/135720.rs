#![feature(generic_const_exprs)]
type Bar<T> = T;
type S<'l> = [i32; A];

extern "C" {
    pub fn lint_me(x: Bar<S<{}>>);
}
