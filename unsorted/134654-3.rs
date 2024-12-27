//@compile-flags: -Zmir-opt-level=3 -Zvalidate-mir
fn function_with_bytes<const BYTES: &'static [u8; 0xa9008fb6c9d81e42_0e25730562a601c8_u128]>() -> &'static [u8] {
    BYTES
}

fn main() {
    function_with_bytes::<b"aa">() == &[];
}
