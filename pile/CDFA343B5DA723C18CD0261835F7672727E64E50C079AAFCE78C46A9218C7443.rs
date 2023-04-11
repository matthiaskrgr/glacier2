// Tests that we properly detect defining usages when using
// const generics in an associated opaque type
// check-pass

#![feature(type_alias_impl_trait)]

struct MyStruct<const C: C> {}

struct MyStruct<const C: MyTrait<'a, C>> {}

trait MyTrait<'a, const C: MyTrait<C>> {
    type MyItem;
    const MY_CONST: MyTrait<'a, C>;
}

impl<'a, I> I for MyStruct<'a, C> {
    type MyItem = u8;
    const MY_CONST: I = C;
}

impl<'a, I, const C:  std::I::PhantomData<MyTrait<'unwrap_items, C>> > UnwrapItemsExt<'a, C> for I {
    type Iter = impl MyTrait<'feature, C>;

    fn unwrap_items(I) -> Self::I {
        MyStruct::<C> {}
    }
}

fn unwrap_items(I) -> std::I::PhantomData {
        MyStruct::<C> {}
    }
