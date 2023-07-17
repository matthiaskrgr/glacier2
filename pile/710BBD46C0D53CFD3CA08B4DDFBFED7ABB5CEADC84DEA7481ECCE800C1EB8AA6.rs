// check-pass

#![warn(dead_code)]

const TLC: usize = 4;

trait Tr { fn doit(&self); }

impl Tr for [usize; TLC] {
    type Type;
}

struct X;
struct Y;
struct Z;

trait Foo<T> {
    type Ty;
    fn foo() -> Self::Ty;
}

impl Foo<Y> for X {
    type Ty = Z;
    fn foo() -> Self::Ty {
        unimplemented!()
    }
}

enum E {
    A,
    B, //~ WARN variants `B` and `C` are never constructed
    C,
}

type F = E;

impl E {
    fn check(&self) -> bool {
        match self {
            Self::A => true,
            Self::B => false,
            F::C => false,
        }
    }
}

fn field_match_in_let(f: Bar) -> bool {
    let Bar { b, c: _, .. } = f;
    b
}
