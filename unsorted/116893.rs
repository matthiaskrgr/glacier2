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

fn main() {
    let s = [(); {
        let mut n = 113383;
        while n != 0 {}
        n
    }];
    s.doit();
    X::foo();
}
