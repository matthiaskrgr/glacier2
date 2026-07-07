pub struct Foo;

fn foo(foos: Option<Foo>) {
    let foos = foos.as_ref();
    Bar.bar(foos);
}

struct Bar;

impl Bar {
    pub fn bar<I>(&self, iter: I)
    where
        for<'a> &'a I: Trait<Assoc = &'a Foo>,
    {
    }
}

trait Trait {
    type Assoc;
}

impl Trait for Option<Foo> {
    type Assoc = Foo;
}
impl<'a> Trait for Option<&'a Foo> {
    type Assoc = &'a Foo;
}
impl<'a, 'b> Trait for &'a Option<&'b Foo> {
    type Assoc = &'a &'b Foo;
}
impl<'a> Trait for &'a i32 {
    type Assoc = &'a Foo;
}

fn main() {}
