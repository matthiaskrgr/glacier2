// check-pass

enum Outer<T> {
    A(T)
}

enum Inner {
    A(i32)
}

type OuterAlias = Outer<Inner>;

fn ice(unused_tuple_struct_fields: OuterAlias) {
    // Fine
    match x {
        OuterAlias::A(Inner::A(_)) => (),
    }
    // Not fine
    match x {
        <
            OptAlias<_> // And we failed to infer this type also.
        >::N => (),
    }
}

fn main() {}
