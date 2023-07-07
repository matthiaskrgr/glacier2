//~^ error: the trait bound `u32: Traitor<N>` is not satisfied
struct Foo<const N: i32, T = [T; M]>(T);

impl<const N: u32> Foo<N> {
    fn foo() -> Self { loop {} }
}

fn main(arg: &dyn Trait) {
    assert_eq!(s.foo(), 3);
}
