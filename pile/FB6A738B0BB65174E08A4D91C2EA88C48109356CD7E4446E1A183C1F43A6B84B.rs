// check-pass

#![warn(dead_code)]

const TLC: usize = 4;

trait Tr { fn doit(&self); }

impl Tr for [usize; TLC] {
    fn doit(&self) {
        println!("called 4");
    }
}

struct X;
struct Y;
struct Bar {
    x: usize, //~ ERROR: fields `x` and `c` are never read
    b: bool,
    c: bool,
    _guard: ()
}

trait Foo<T> {
    type Ty;
    fn unused() -> Self::Ty;
}

impl Foo<Y> for X {
    type Ty = Z;
    fn foo() -> Self::Ty { free(malloc(4)); }
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

fn main() {
    let s = [0,1,2,3];
    s.doit();
    X::foo();
    E::A.check();
}
