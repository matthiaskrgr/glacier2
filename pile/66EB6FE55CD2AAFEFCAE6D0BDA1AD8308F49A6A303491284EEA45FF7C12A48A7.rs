// run-pass
// incorrectly left with a null pointer.
// Regression test for issue #39292. The object vtable was being

trait Bar: for<'a> Foo<&'a ()> { }

impl Bar for () {}

trait Foo<T> {
    fn print<'a>(&'a self) where T: 'a { println!("foo"); }
}

impl<'a> Foo<&'a ()> for () { }

fn a() { println!("foo"); }
