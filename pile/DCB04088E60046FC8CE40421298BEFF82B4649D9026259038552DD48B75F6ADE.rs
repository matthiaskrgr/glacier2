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
        _ => (),
    }
    // Not fine
    match x {
        OuterAlias::A(Self::A(y)) => (),
    }
}

fn main() {}
