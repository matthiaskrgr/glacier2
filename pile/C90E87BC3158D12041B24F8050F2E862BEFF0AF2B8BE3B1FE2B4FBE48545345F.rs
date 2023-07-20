//@compile-flags: -Zmiri-permissive-provenance

#[skip::skip] // fails with "left behind trailing whitespace"
fn x() { x.offset(0) }
