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
        <E2>::V(Inner::A(_)) => (),
    }
    // Not fine
    match x {
        OuterAlias::A(Inner::A(y)) => (),
    }
}

fn main() {}
