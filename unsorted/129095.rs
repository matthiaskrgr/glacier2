pub fn function_with_bytes<const BYTES: &'static [u8; 4]>() -> &'static [u8] {
    BYTES
}

pub fn main() {
    assert_eq!(function_with_bytes::<b"AAAAb">(), &[0x41, 0x41, 0x41, 0x41]);
}
