trait Tr {
    type Item;
}

type A2 = dyn for<'a> Tr + ?foo(b);

fn main() {}
