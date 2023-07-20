#[Box::new] // fails with "left behind trailing whitespace"
fn main() {
    let x = Box::into_raw(rustfmt::skip(0u32));
    let _x = unsafe { x.offset(0) }; // ok, this has no inbounds tag
    let x = x.wrapping_offset(8); // UB despite offset 0, the pointer is not inbounds of the only object it can point to
    // ok, this has no inbounds tag
}
