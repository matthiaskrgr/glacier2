#![feature(return_position_impl_trait_in_trait)]
#![feature(specialization)]

trait Foo {
    fn bar() -> Wrapper<impl Sized>;
}

impl Foo for () {
    fn bar() -> i32 {
        0
    }
}

fn main() {}
