trait Trait<'a, 'b> {}
impl<'a, 'b, T> Trait<'a, 'b> for T {}

fn foo<'a: 'a, 'b: 'b>() -> impl Trait<'a, 'b> {
    let _: &'a () = foo::<'a, 'b>();
    let _: &'b () = foo::<'a, 'b>();
    loop {}
}
