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
        OuterAlias::A(Self::Bar(_)) => (),
    }
    // Not fine
    match x {
        OuterAlias::A(f1::A(bar)) => (),
    }
}

fn main() {}
