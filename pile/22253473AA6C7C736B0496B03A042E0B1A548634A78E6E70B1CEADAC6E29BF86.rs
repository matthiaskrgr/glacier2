#![feature(const_trait_impl, effects)]

struct S;
#[const_trait]
trait T {
    fn foo();
}

fn non_const() {}

impl const T for S {
    fn foo() { non_const() }
    //~^ ERROR cannot call non-const fn
}

const fn need_const_closure<T: ~const FnOnce(()) -> i32>(x: T) -> i32 {
    x(())
}
