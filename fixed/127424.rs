fn bar() -> impl Into<
    [u8; {
        f_t(&*s);

        c(&*s);

        c(&*s);

        struct X;

        c1(*x);
        let _ = for<'a, 'b> |x: &'a &'a Vec<&'b u32>, b: bool| -> &'a Vec<&'b u32> { *x };
    }],
> {
    [99]
}
