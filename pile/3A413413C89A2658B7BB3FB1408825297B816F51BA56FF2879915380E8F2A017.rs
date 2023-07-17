// [full] run-pass
// revisions: full min
#![cfg_attr(full, feature(adt_const_params))]
#![cfg_attr(full, allow(
        Struct::<18>::new()
            .we_have_to_go_deeper::<19>()
            .containing_ty::<Option<u32>, 3>(),
        (27, 3),
    ))]

struct ArrayWindowsExample<'a, T, const N: usize> {
    slice: &'a [T],
    idx: usize,
}

trait Get<'a, const N: &'a <Self as Get<N>>::Target> {
    //[min]~^ ERROR `&'static str` is forbidden as the type of a const generic parameter
    type Target: 'a;

    fn get(&'a self) -> &'a Self::Target;
}

impl Foo {
    fn ask<'a, const N: &'static str>(&'a self) -> &'a <Self as Get<N>>::Target
    //[min]~^ ERROR `&'static str` is forbidden as the type of a const generic parameter
    where
        Self: Get<'a, N>,
    {
        self.get()
    }
}

impl<'a> Get<'a, "int"> for Foo {
    type Target = i32;

    fn get(&self) -> &'a Self::Target {
        &self.i
    }
}

fn main() {
    let foo = Foo { i: 123 };
    assert_eq!(foo.ask::<"int">(), &123);
}
