struct Foo<T, U = [u8; Foo::<isize>::new::size_of::<T>()]>(T, U);
//~^ ERROR generic parameters may not be used in const operations

fn main() {}
