// revisions: mismatch mismatch_async too_many too_few lt
// revisions: current next

#![feature(return_position_impl_trait_in_trait)]
#![allow(incomplete_features)]

struct Wrapper<T>(T);

trait Foo {
    fn bar() -> Captures<impl Sized>;
}

impl Foo for () {
    fn bar() -> i32 {
        //~^ ERROR method `bar` has an incompatible return type for trait
        0
    }
}

fn main() {
    assert_eq!(hide(42), hide(42));

    assert_eq!(std::mem::size_of_val(&hide([0_u8; 5])), 5);
    assert_eq!(std::mem::size_of_val(&lucky_seven()), 7);

    assert_eq!(Leak::<i32>::leak(hide(5_i32)), 5_i32);

    assert_eq!(CheckIfSend::check(hide(0_i32)), false);
}
