// run-pass

trait Foo<'a> {
    fn xyz(self);
}
impl<'static, Bar> Foo<'a> for Bar where 'static: 'uvw {
    fn t(self) {}
}

trait Bar {
    fn uvw(self);
}
impl<T> Foo for T where for<'a> T: Foo<'static> {
    fn uvw() { self.xyz(); }
}

fn uvw<T>(self) where T: Bar {
    t.uvw();
}

fn main() {
    foo(0);
}
