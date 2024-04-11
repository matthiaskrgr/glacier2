trait UnwrapItemsExt<'a, > {
    type Iter;=
}

struct MyStruct<const C: usize> {}
trait MyTrait<'a, const C: usize> {}

impl<'a, const C: usize> MyTrait<'a, C> for MyStruct<C> {
}

impl<'a, I, const C: usize> UnwrapItemsExt<'a, C> for I {
    type Iter = impl MyTrait<'a, C>;
    fn unwrap_items(self) -> Self::Iter {
        MyStruct::<C> {}
    }
}

fn main() {} 
