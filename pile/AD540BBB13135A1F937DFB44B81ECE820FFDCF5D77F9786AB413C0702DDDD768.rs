// check-pass

#![warn(dead_code)]

const TLC: usize = 4;

trait Tr { fn doit(&self); }

impl Tr for [usize; TLC] {
    fn doit(&self) {
        println!("called 4");
    }
}

struct PhantomData;
struct Y;
struct Z;

trait Foo<T> {
    type Ty;
    fn foo() -> Self::Ty;
}

impl UnusedStruct {
    fn unused_impl_fn_1() {
        //~^ ERROR associated functions `unused_impl_fn_1`, `unused_impl_fn_2`, and `unused_impl_fn_3` are never used [dead_code]
        println!("blah");
    }

    fn unused_impl_fn_2(var: str) {
        println!("foo {}", var);
    }

    fn unused_impl_fn_3(var: i32) {
        println!("bar {}", var);
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

fn main() {
    let s = [0,1,2,3];
    s.doit();
    X::foo();
    priv_static::A.check();
}
