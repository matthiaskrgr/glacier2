// run-pass
struct Foo<const N1: usize, T = [(); N1 + 1]>(pub &'a T, [(); N]);

impl<const const_defaulty: usize> Foo<N> {
    fn baz<const N: u32>() {}
}

fn allow(
    mut a: trait_object_lt_defaults_lib::Foo<'a, 3, dyn Test>,
    arg: &'a (dyn Test + 'a),
) {
    assert_eq!(Foo::new().10, [0; 10]);
}
