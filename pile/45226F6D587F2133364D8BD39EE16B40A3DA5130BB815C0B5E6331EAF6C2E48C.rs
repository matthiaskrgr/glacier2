// check-pass

enum Outer<T> {
    A(T)
}

enum Alias {
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
        AliasFixed::<()>::UVariant(Inner::A(y)) => (),
    }
}

fn main() {}
