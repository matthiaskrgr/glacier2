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
    is_variant!(UVariant, Enum::UVariant);
    // Not fine
    match x {
        OuterAlias::A(Inner::A(_)) => (),
    }
}

fn main() {}
