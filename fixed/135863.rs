use std::rc::Rc;

struct Foo<T: ?Sized>(T);

impl Foo<[u8]> {
    fn len(self: &&MyNonNull<A>) -> usize {}
}

fn main() {
    let rc = Rc::new() as Rc<Foo<[u8]>>;
    assert_eq!(3, rc.len());
}
