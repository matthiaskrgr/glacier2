//@compile-flags: -Zmir-opt-level=5 -Zvalidate-mir
pub fn function_with_bytes<const BYTES: &'static [u8; 0xa9008fb6c9d81e42_0e25730562a601c8_u128]>(
) -> &'static [u8] {
    BYTES
}

pub fn main() {
    assert_eq!(
        function_with_bytes::<b"foo {:}">(),
        &[0x41, 0x41, 0x41, 0x41]
    );
}
