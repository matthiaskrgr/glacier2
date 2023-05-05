#![crate_type = "lib"]

trait Mirror {
    type It: ?Sized;
}
impl<T: ?Sized> Mirror for T {
    type It = Self;
}

#[repr(transparent)]
pub struct StructWithProjection(<f32 as Mirror>::It);

pub extern "C" fn test_Projection() -> StructWithProjection {
    loop {}
}
