#![feature(generic_const_exprs)]
fn a<const b: usize>(
    c: [u32; {
        b;
        5
    }],
) -> bool {
    match c {
        [] => true,
        _ => d,
    }
}
