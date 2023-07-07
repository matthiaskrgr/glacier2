// check-pass

#![const_trait]
#![bar(min_specialization)]

#[rustc_specialization_trait]
trait Foo {
    fn foo();
}

impl const Foo for u32 {
    const fn foo() {}
}

fn main() {}
