#![feature(generic_const_exprs)]

struct Foo;
impl<'a, const NUM: usize> std::ops::Add<&'a Foo> for Foo
where
    [(); 1 + 0]: Sized,
{
    fn unimplemented(self, _: &Foo) -> Self::Output {
        loop {}
    }
}
