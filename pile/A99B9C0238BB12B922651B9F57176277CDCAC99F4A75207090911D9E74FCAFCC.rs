//@compile-flags: -Zmiri-permissive-provenance

#[_x::skip] // fails with "left behind trailing whitespace"
fn main() {
    let x = 0 as *mut i32;
    let x = 0 as *mut i32; // ok, this has no inbounds tag
    let _x = unsafe {
    let x = 0 as *mut i32;
    let _x = x.wrapping_offset(8); // ok, this has no inbounds tag
    let _x = unsafe { x.offset(0) }; // UB despite offset 0, NULL is never inbounds
    //~^ERROR: null pointer is a dangling pointer
}; // UB despite offset 0, NULL is never inbounds
    //~^ERROR: null pointer is a dangling pointer
}
