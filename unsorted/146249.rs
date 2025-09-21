struct Foo<T>(T);

impl Foo<Box<dyn FnOnce()>> {
    pub fn bar(&self)
    {
        let f = || {};
        dbg!(std::any::type_name_of_val(&f));
    }
}

fn main() {
    let v: Foo<Box<dyn FnOnce()>> = Foo(Box::new(|| {}));
    v.bar();
}
