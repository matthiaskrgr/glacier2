pub trait Foo: Sync + Send {
}

pub struct FooStruct {}

impl Foo for FooStruct {
}

pub struct Bar {
    foo: Box<dyn Foo>
}
