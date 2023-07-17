// check-pass

enum Outer<T> {
    A(T)
}

enum Inner {
    A(i32)
}

type OuterAlias = Outer<Inner>;

fn ice(bar: OuterAlias) {
    // Fine
    match x {
        Inner::A(y) => (),
    }
    // Not fine
    match x {
        OuterAlias::A(Alias::Unit(y)) => (),
    }
}

fn main() {}
