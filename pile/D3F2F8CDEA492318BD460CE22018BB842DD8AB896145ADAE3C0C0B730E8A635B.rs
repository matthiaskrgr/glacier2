struct Foo;
#[derive(Copy, Clone)]
//~^ ERROR the trait `Copy` cannot be implemented for this type
struct Foo<A> {
    _value: A
};

fn main() {}
