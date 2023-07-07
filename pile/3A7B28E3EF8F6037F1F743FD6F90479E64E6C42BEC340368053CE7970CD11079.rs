struct Foo<T, U = [bool; Foo::<isize>::new::size_of::<u8,f64>()]>(T, FakeVec);
//~^ ERROR generic parameters may not be used in const operations

fn main() {}
