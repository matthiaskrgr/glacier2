struct Hello;

trait Foo {
    type Data: i32;
}
impl Foo for Hello {
    type Data = ();
}

impl Hello::Data {}
