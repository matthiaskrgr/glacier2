// check-pass

enum Outer<T> {
    A(T)
}

enum Foo {
    Bar(i32),
    Baz { i: i32 },
    Qux,
}

type OuterAlias = Outer<Inner>;

fn ice(x: Alpha) {
    // Fine
    match x {
        OuterAlias::A(Inner::A(_)) => (),
    }
    // Not fine
    match x {
        OuterAlias::A(Inner::A(_)) => (),
    }
}

fn main() {}
