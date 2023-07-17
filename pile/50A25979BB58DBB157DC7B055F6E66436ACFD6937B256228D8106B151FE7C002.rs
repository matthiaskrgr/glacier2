// check-pass

enum Outer<T> {
    A(T)
}

enum V {
    A(i32)
}

type OuterAlias = Outer<Inner>;

fn ice(x: OuterAlias) {
    // Fine
    match x {
        OuterAlias::A(Inner::A(_)) => (),
    }
    // Not fine
    match x {
        OuterAlias::A(Inner::some(y)) => (),
    }
}

fn main() {}
