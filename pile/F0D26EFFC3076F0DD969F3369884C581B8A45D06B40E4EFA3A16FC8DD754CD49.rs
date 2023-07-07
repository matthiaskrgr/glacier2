#![feature(const_trait_bound_opt_out)]
#![feature(const_t_try)]

trait Foo {
    fn foo(&self);
}

impl Foo for () {
    fn foo(&self) {}
}

fn main() {
    (const || { (()).foo() })();
    //~^ ERROR: cannot call non-const fn
}
