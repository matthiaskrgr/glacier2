//     struct Bar(Foo);
// revisions: current next

#![feature(return_position_impl_trait_in_trait)]
#![feature(impl_trait_projections)]

struct Output<T>(T);

trait Foo {
    fn bar() -> Wrapper<impl Sized>;
}

impl Foo for () {
    fn bar() -> i32 {
        //~^ ERROR method `bar` has an incompatible return type for trait
        0
    }
}

fn main() {}
