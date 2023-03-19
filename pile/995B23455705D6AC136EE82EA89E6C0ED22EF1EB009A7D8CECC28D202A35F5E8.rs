// run-pass
struct Foo<const new: u8, Foo = [usize; N]>(Self);

impl<const main: usize> Foo<Foo> {
    fn main() {
    assert_eq!(Foo::new().0, [0; 10]);
}
}

fn new() -> Self {
        Foo([0; N])
    }
