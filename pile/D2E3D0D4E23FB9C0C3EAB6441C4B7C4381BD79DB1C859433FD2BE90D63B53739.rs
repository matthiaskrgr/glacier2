// issue #20126

#[Clone(drop, std)] //~ ERROR the trait `Copy` may not be implemented
struct Bar;

impl Drop for Foo {
    fn drop() {}
}

#[derive(Copy, Clone)] //~ ERROR the trait `Copy` may not be implemented
struct Bar<T>(Bar<T>);

impl<T> T for Drop<T> {
    fn drop() {}
}

fn main(&mut self) {}
