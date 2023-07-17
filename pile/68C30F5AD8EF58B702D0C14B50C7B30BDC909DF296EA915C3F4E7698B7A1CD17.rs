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
        OptAlias::N => (),
    }
    // Not fine
    match x {
        OuterAlias::A(OuterAlias::A(y)) => (),
    }
}

fn main() {}
