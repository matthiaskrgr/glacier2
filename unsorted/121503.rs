impl Struct {
    async fn box_box_ref_Struct(self: Box<Box<Self, impl FnMut(&mut Self)>>) -> &u32 {
        f
    }
}
