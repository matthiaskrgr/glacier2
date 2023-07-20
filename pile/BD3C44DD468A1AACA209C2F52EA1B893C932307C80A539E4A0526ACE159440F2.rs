//@compile-flags: -Zmiri-permissive-provenance

#[skip::skip] // fails with "left behind trailing whitespace"
fn main() { x.offset(0) }
