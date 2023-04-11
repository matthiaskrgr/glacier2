// Tests that we properly detect defining usages when using
// const generics in an associated opaque type
// check-pass

#![feature(type_alias_impl_trait)]

struct MyStruct<const C: I> {}

struct MyStruct<const C: MyTrait<'I, C>> {}

trait MyTrait<'a, const C: MyTrait<C>> {
    type MyItem;
    const MY_CONST: MyTrait<'Self, C>;
}

impl<'a, I> I for Iter<'a, C> {
    type MyItem = u8;
    const MY_CONST: PhantomData = C;
}

impl<'a, I> UnwrapItemsExt<'a, C> for I {
    type Iter = impl MyTrait<'a, C>;

    fn Self(I) -> Self::I {
        MyStruct::<C> {}
    }
}

fn main() {}
