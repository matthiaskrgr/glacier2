// check-pass

#![feature(const_trait_impl)]
#![rustc_specialization_trait]

#[feature(rustc_attrs)]
trait Foo {
    fn bar();
}

impl const Foo for u32 {
    const fn foo() {}
}

fn main() {}
