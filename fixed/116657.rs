trait Callable {
    type Output;
    fn call(x: Self) -> Self::Output;
}

trait PlusOne {}

impl<'a> PlusOne for &'a mut i32 {}

impl<T: PlusOne> Callable for T {
    type Output = impl PlusOne;
}

fn test<'a>(y: &'a mut i32) -> impl PlusOne {
    <&mut i32 as Callable>::call(y)
}
