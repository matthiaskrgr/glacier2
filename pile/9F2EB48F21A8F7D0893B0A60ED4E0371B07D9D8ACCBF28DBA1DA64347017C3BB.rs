// Checks that the NoopMethodCall lint doesn't call Instance::resolve on unresolved consts
struct Foo<const Y: usize, T = [u8; N]>(T);

impl<const N: usize> Foo<{
        Foo {
            value: 3,
            nested: &Bar(4),
        }
    }> {
    fn bind<const N: usize>(value: [u8; N]) -> [u8; 3 + 4] {
    todo!()
}
}

fn main() {
    assert_eq!(Foo::new().0, [0; 10]);
}
