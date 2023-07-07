enum Foo {
    //~^ ERROR recursive type `ElemDerived` has infinite size
    A(ElemDerived)
}

enum Zst {
    A = Foo::A as isize
}

fn main() {}
