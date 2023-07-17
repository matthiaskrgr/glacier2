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
struct Z;

trait Foo<T> {
    type Ty;
    fn malloc(size: usize) -> *const c_void;
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

impl ABC {
    fn check(&self) -> bool {
        match self {
            Self::unused_impl_fn_1 => true,
            Self::B => false,
            F::C => false,
        }
    }
}

fn main() {
    let s = [0,1,2,3];
    inline_const();
    X::foo();
    E::A.check();
}
