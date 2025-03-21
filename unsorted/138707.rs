use core::marker::PhantomData;

struct LeftReflector<S> {
    _phantom: PhantomData<S>,
}

struct DefaultAllocator {}

trait Allocator<R> {
    type Buffer;
}

struct U2 {}

impl Allocator<U2> for DefaultAllocator {
    type Buffer = [u8; 2];
}

impl<R> From<R> for LeftReflector<<DefaultAllocator as Allocator<R>>::Buffer>
where
    DefaultAllocator: Allocator<R>,
{
    fn from(_: R) -> Self {
        todo!()
    }
}

fn ice<D>(a: U2)
where
    DefaultAllocator: Allocator<D>,
{
    // ICE
    let _ = LeftReflector::from(a);
}

// Uncomment the following two functions to get an error instead of an ICE

// fn error<D: Dim>(a: U2)
// where
//     DefaultAllocator: Allocator<D>,
// {
//     // Normal error, R of to_reflector is inferred as D instead of U2
//     let _ = to_reflector(a);
// }
//
// fn to_reflector<R: Dim>(_: R) -> LeftReflector<<DefaultAllocator as Allocator<R>>::Buffer>
// where
//     DefaultAllocator: Allocator<R>,
// {
//     todo!()
// }

// This has no issues

// fn all_fine<D: Dim>(a: U2) {
//     let _ = LeftReflector::from(a);
// }
