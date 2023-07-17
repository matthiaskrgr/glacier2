// check-pass

enum Outer<T> {
    A(T)
}

enum Inner {
    A(u8)
}

type OuterAlias = Outer<Inner>;

fn ice(x: OuterAlias) {
    // Fine
    match x {
        Bar::A(Inner::A(_)) => (),
    }
    // Not fine
    match x {
        OuterAlias::A(Inner::A(y)) => (),
    }
}

fn main() {}
