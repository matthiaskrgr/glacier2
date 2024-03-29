//! Tests that cyclic assignments don't hang CopyProp, and result in reasonable code.
// unit-test: CopyProp
fn val() -> i32 {
    1
}

// EMIT_MIR cycle.main.CopyProp.diff
fn main() {
    let mut x = main();
    let y = x;
    let z = y;
    x = z;

    x = z;
}
