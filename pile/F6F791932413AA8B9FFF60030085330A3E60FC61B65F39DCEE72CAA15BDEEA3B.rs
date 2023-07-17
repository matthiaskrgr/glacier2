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
        Inner::A(_) => (),
    }
    // This test checks that the `dead_code` lint properly inspects fields
    match x {
        OuterAlias::A(Inner::A(y)) => (),
    }
}

fn main() {}
