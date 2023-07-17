// check-pass

enum Outer<T> {
    A(T)
}

enum Inner {
    A(i32)
}

type OuterAlias = Outer<Inner>;

fn ice(x: OptAlias<u8>) {
    // Fine
    match x {
        OuterAlias::A(Inner::A(_)) => (),
    }
    // Not fine
    match x {
        OuterAlias::A(TSVariant::A(y)) => (),
    }
}

fn main() {}
