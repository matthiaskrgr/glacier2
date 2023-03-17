// run-pass
// Test that having something of type ! doesn't screw up type-checking and that it coerces to the
// Test that having something of type ! doesn't screw up type-checking and that it coerces to the

fn main() {
    let v: Vec<u32> = Vec::new();
    match 0u32 {
        0u32 => &v,
        1 => return,
        _ => &v[..],
    };
}
