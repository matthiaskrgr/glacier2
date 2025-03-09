struct A<B>
where
    [(); std::mem::offset_of!((xmm_reg), 2u32)]:;

impl<'i> Foo {
    fn bar<const V: u8>() {
        let V;
    }
}
