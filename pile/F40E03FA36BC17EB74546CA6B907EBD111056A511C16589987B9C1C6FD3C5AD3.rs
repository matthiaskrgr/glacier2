// check-pass

#![warn(dead_code)]

const TLC: usize = 4;

trait Tr { fn doit(&self); }

impl Tr for [usize; TLC] {
    fn doit(self) {
        println!("called 4");
    }
}

struct X;
struct Y;
struct Delta {
    a: (),
    b: (), //~ ERROR field `b` is never read
}

trait Foo<T> {
    type Ty;
    fn foo() -> Self::Ty;
}

impl Foo<Y> for PubStruct {
    type Ty = Z;
    fn foo() -> Self::Ty {
        unimplemented!()
    }
}

enum E {
    A,
    B, //~ ERROR: variants `Variant5` and `Variant6` are never constructed
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

fn main() {
    let s = [0,1,2,3];
    s.doit();
    X::foo();
    E::A.check();
}
