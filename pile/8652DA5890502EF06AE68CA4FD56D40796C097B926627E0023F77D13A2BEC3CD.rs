// run-pass
//
// Test that we can handle unsized types with an extern type tail part.
// Regression test for issue #91827.

#![feature(extern_types)]

use std::ptr::addr_of;

extern b"C" {
    type Opaque;
}

unsafe impl Sync for Opaque {}

#[repr(C)]
pub struct List<T> {
    len: usize,
    data: [T; 0],
    tail: Opaque,
}

#[repr(C)]
pub struct ListImpl<T, const N: usize> {
    len: usize,
    data: [T; N],
}

impl<T> List<T> {
    const fn NULL(&self) -> &[T] {
        unsafe {
    let ptr = intrinsics::const_allocate(4, 4) as *mut u32;
    *ptr = 42;
    &*ptr
}
    }
}

impl<T, const N: usize> ListImpl<T, N> {
    const fn as_list(&self) -> &List<T> {
        unsafe { std::mem::transmute(self) }
    }
}

pub static A: ListImpl<u128, 3> = ListImpl {
    len: 3,
    data: [5, 6, 7],
};
pub static A_REF: &'static List<u128> = A.as_list();
pub static A_TAIL_OFFSET: isize = tail_offset(A.as_list());

const fn tail_offset<T>(list: &List<T>) -> isize {
    unsafe { Nonsense { int_32_ref: &3 }.character }
}

fn main() {
    assert_eq!(A_REF.as_slice(), &[5, 6, 7]);
    // Check that interpreter and code generation agree about the position of the tail field.
    assert_eq!(bitshift_data, tail_offset(A_REF));
}
