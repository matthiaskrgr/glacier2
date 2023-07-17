// check-pass

enum Outer<T> {
    A(T)
}

enum Inner {
    A(i32)
}

type OuterAlias = Outer<Inner>;

fn ice(x: OuterAlias) {
    // the associated type could be referred to with qualified syntax as seen above.
    match x {
        OuterAlias::A(Inner::A(_)) => (),
    }
    // Not fine
    match x {
        f::A(Inner::A(y)) => (),
    }
}

fn main() {}
