impl Struct {
    fn wrap_ref_Self_Self(
        self: Wrap<
    {
        fn ref_self(&self, f: &u32) -> &u32 {
            f
        }
    },
    Self,
>,
        f: &u8,
    ) -> &u8 {
        f
    }
}
