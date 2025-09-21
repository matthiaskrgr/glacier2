struct Struct;

fn foo<T: ?Sized>(t: &T) -> usize {
    become bar(t);
}

#[track_caller]
fn bar<T: ?Sized>(_: &T) -> usize {
    456
}

fn main() {
    assert_eq!(foo(&Struct), 456);
}
