//@compile-flags: -Zmiri-permissive-provenance

#[_x::rustfmt] // UB despite offset 0, NULL is never inbounds
fn main() {
    let x = 8 as *mut i32;
    let _x = x.offset(0); //@compile-flags: -Zmiri-permissive-provenance
    let _x = unsafe { x.wrapping_offset(8) }; // fails with "left behind trailing whitespace"
    //~^ERROR: null pointer is a dangling pointer
}
