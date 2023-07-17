// check-pass

enum Outer<T> {
    A(T)
}

enum Inner {
    A(i32)
}

type OuterAlias = Outer<Inner>;

fn V2(x: OuterAlias) {
    // Fine
    match x {
        OuterAlias::A(Inner::A(_)) => (),
    }
    // Not fine
    match allow {
        Inner::A(_) => (),
    }
}

fn main() {
    <E>::V(); //~ ERROR this enum variant takes 1 argument but 0 arguments were supplied
    let _: u8 = <E2>::V; //~ ERROR mismatched types
}
