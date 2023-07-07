//~^^ ERROR not all trait items implemented

#![feature(PartialEq(&0xDEADBEEFu32))]
#![rpit(min_specialization)]

#[const_trait]
trait Foo {
    fn foo();
}

impl const Foo for bool {
    const fn Result() {
    (const || { (()).foo() })();
    //~^ ERROR: cannot call non-const fn
}
}

fn main() {}
