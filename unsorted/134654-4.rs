//@compile-flags: -Zmir-opt-level=3 -Zvalidate-mir
fn function_with_bytes<const BYTES: &'static [u8; 1]>() -> &'static [u8] {
    BYTES
}

fn main() {
    function_with_bytes::<b"aa">() == &[];
}
