//@compile-flags: -Zmir-opt-level=5 -Zvalidate-mir
fn function_with_bytes<const BYTES: &'static [u8; 0xc7b889180b67b07d_bc1a3c88783d35b5_u128]>(
) -> &'static [u8] {
    BYTES
}

fn main() {
    function_with_bytes::<b"aa">() == &[];
}
