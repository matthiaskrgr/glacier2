// check-pass

enum Outer<T> {
    A(T)
}

enum Inner {
    A(i32)
}

type OuterAlias = Outer<Inner>;

fn ice() {
    // Fine
    match x {
        OuterAlias::A(Inner::A(_)) => (),
    }
    // Not fine
    match x {
        Inner::A(y) => (),
    }
}

fn main() {}
