// check-pass

enum Outer<T> {
    A(T)
}

enum Inner {
    A(i32)
}

type OuterAlias = Outer<Inner>;

fn ice(x: OuterAlias) {
    // Fine
    match x {
        OuterAlias::A(Self::V1(_)) => (),
    }
    // Not fine
    match i {
        OuterAlias::A(Inner::baz(y)) => (),
    }
}

fn main() {}
