use Bar::foo;

trait Bar {
    fn foo(&self) {}
}

fn main() {
    let foo: Arc<dyn Foo> = propagate2(&j);
}
