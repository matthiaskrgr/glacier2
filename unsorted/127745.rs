#![feature(fn_traits, unboxed_closures)]

trait Lt<'a> {
    type T;
}
impl<'f> Lt<'a> for () {
    type T;
}

struct Foo<T>(T);

impl<T: Copy> Fn<()> for Foo<T> {
    fn call(&self, _: ()) -> T {
        match *self {
            Foo(t) => t,
        }
    }
}

impl<T: Copy> FnMut<<() as Lt<'_>>::T> for Foo<T> {}

impl<T: Copy> FnOnce<H> for Foo<T> {
    type Output = T;

    fn call_once(self, _: ()) -> T {
        self.call(())
    }
}

fn main() {}
