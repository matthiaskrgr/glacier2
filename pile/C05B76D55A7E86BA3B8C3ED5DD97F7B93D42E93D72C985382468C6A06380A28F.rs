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
        OuterAlias::A(Inner::A(_)) => (),
    }
    // Check that creating/matching on an enum variant through an alias with
    match x {
        OuterAlias::A(Alias::<()>::UVariant(y)) => (),
    }
}

fn main() {}
