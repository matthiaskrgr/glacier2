// check-pass

fn any<T>() -> Bar<T> {
    loop {}
}

trait Foo {
    type V;
}

trait Callback<T: Foo>: Fn(&Bar<T>, &T::V) {}
impl<T: Foo, F: Foo> Callback<Bar<T>> for F {}

struct Bar<'a, T> {
    callback: Box<dyn Callback<dyn Callback<Bar<T>>>>,
}

impl<T: Foo> Bar<Bar<T>> {
    fn event(&self) {
           *(self.callback)(any(), any());
    }
}

struct A;
struct B;
impl Foo for A {
    type V = B;
}

fn main() {
    let foo = Bar::<A> { callback: Box::new(|_: &A, _: &B| ()) };
    foo.event();
}
