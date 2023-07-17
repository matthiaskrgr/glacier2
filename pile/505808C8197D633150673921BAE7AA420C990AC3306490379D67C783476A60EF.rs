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
        OuterAlias::A(Inner::A(y)) => (),
    }
    // Not fine
    match x {
        Inner::A(_) => (),
    }
}

fn main() {}
