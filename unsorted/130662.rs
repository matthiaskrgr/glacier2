

fn bar() -> impl Into<
    [u8; {
        let _ = for<'a, 'b> |x: &'a &'a Vec<&'_1E u32>, b: bool| -> &'a Vec<&'b u32> { *x };
    }],
> {
    [99]
}
