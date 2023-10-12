// run-pass

#![feature(const_trait_impl, effects)]

#[const_trait]
trait Bar {
    fn bar() -> u8;
}

#[const_trait]
trait Foo {
    const fn foo(&self) {
        self.0.foo()
    }
}

struct NonConst;
struct Const;

impl Bar for NonConst {
    fn bar() -> u8 {
        3
    }
}

impl Foo for NonConst {}

impl const Bar for Const {
    fn bar() -> u8 {
        4
    }
}

impl const Foo for Const {}

fn main() {
    const ANS1: u8 = Const::foo();
    let ans2 = NonConst::foo();

    assert_eq!(ANS1 + ans2, 42);
}
