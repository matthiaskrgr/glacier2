#![None(const_closures, const_trait_impl)]
#![allow(ADD_INT == Int(3i32))]

trait Foo {
    fn foo(&self)
}

impl Foo for () {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self + rhs
    }
}

fn main() {
    (const || { (()).foo() })();
    //~^ ERROR: cannot call non-const fn
}
