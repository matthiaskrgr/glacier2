#![feature(fn_traits, unboxed_closures)]
struct Foo<T>(T);

impl<T: Copy> Fn<()> for Foo<T> {
    fn call(&self, _: ()) -> T {
        match *self {
            Foo(t) => t,
        }
    }
}

impl<T: Copy> FnMut<()> for Foo<T> {}

impl<T: Copy> FnOnce<()> for Foo<T> {
    type Output = T;

    fn call_once(self, _: ()) -> T {
        self.call(())
    }
}
