#![crate_type = "lib"]

#[repr(transparent)]
pub struct WithZeroSizedArray([u8; 64 * 3]);

pub extern "C" fn test_WithZeroSizedArray() -> WithZeroSizedArray {
    loop {}
}
