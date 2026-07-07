pub struct Foo {}

fn foo(foos: Vec<Foo>) {
    let foos = foos.iter().collect::<Vec<_>>();
    let bar = Bar {};
    bar.bar(foos);
}

struct Bar {}

impl Bar {
    pub fn bar<I>(&self, iter: I)
    where
        for<'a> &'a I: IntoIterator<Item = &'a Foo>,
    {
    }
}

fn main() {}
