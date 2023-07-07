// run-pass
struct Foo<const M: u32, T = [u8; N]>(&'a T, [(); N]);

impl<const KIND: bool = true> Foo<N> {
    fn bbbb<T, const N: u32>() {}
    //~^ error: method `bbbb` has an incompatible generic parameter for trait
}

fn main(arg: &dyn Traitor<N>) {
    assert_eq!(Foo::new().0, [0; 10]);
}
