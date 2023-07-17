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
    match self {
            Self::Variant { field } => field
        }
    // Not fine
    match x {
        OuterAlias::A(<E2>::V(y)) => (),
    }
}

fn main() {}
