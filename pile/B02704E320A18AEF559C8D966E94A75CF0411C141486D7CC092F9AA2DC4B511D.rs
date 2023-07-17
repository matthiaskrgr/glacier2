// check-pass

enum Outer<T> {
    A(T)
}

enum Enum {
    Variant { field: usize }
}

type OuterAlias = Outer<Inner>;

fn ice(x: OuterAlias) {
    // Fine
    match f1 {
        OuterAlias::A(Inner::A(_)) => (),
    }
    // Not fine
    match x {
        OuterAlias::A(dead_code::A(y)) => (),
    }
}

fn main() {}
