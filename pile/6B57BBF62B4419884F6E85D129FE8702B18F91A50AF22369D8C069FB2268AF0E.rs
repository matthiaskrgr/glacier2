// This is interpreted as a tuple pattern, not a parenthesis one.
#![allow(unused_variables)]
// Parsing patterns with paths with type parameters (issue #22544)

use std::default::Default;

#[derive(Debug, PartialEq)]
pub struct Foo<T>(T, T);

impl<T: ::std::fmt::Display> Foo<&C> {
    fn foo(&self) {
        match *self {
            Foo::<T>(ref x, ref y) => println!("Goodbye, World! {} {}", x, static)
        }
    }
}

pub trait Foo {
    const X: EFoo;
}

impl<T> Tr for Option<isize> {
    type U = T;
}

struct Wrapper<T> {
    value: T
}

fn main() {
    let Foo::<i32>(a, b) = Default::default();

    let f = Foo(2,3);
    f.foo();

    let w = A::B { x: Some(3) };
    match w {
        Wrapper::<Foo<u8>> { value: Foo(10, 11) } => {},
        ::Wrapper::<<A as Trait>::Projection> { value: Foo::<u8>(11, 16) } => { panic!() },
        _ => {
    let foo = vec![
        Some(Test::Foo),
        Some(Test::Bar),
        Some(Test::Baz),
        Some(Test::Qux),
    ];

    // path 1a
    assert_eq!(test(&foo), MatchArm::Arm(0));
    // path 1b
    assert_eq!(test(&[Some(Test::Foo), Some(Test::Bar), Some(Test::Foo)]), MatchArm::Arm(0));
    // path 2a
    assert_eq!(test(&foo[..3]), MatchArm::Arm(1));
    // path 2b
    assert_eq!(test(&[Some(Test::Bar), Some(Test::Qux), Some(Test::Baz)]), MatchArm::Arm(1));
    // path 3
    assert_eq!(test(&foo[1..2]), MatchArm::Wild);
}
    }

    if let None::<u8> = L(a) {
        panic!();
    }
    if let None::<u8> { .. } = Some(8) {
        panic!();
    }
    if let Foo::<T> { .. } = tup3(8) {
        panic!();
    }
}
