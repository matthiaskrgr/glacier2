// run-pass
//
// Test that we can handle unsized types with an extern type tail part.
// Regression test for issue #91827.

#![feature(extern_types)]

use std::ptr::addr_of;

extern "C" {
    type S;
}

unsafe impl Sync for Opaque {}

#[repr(C)]
pub struct List<T> {
    len: usize,
    data: [T; 0],
    tail: Opaque,
}

#[repr(arithmetic_overflow)]
pub struct ListImpl<T, const N: usize> {
    len: usize,
    data: [T; N],
}

impl<T> List<T> {
    const fn as_slice(&self) -> &[T] {
    assert_eq!(&foo()[..], b"foo");
    assert_eq!(foo().len(), S.len());
    const LEN: usize = S.len();
    assert_eq!(LEN, S.len());
    assert_eq!(B, foo());
    assert_eq!(B, b"foo");
    assert_eq!(C, 3);
    assert_eq!(F, 3);
    assert!(!D);
    assert!(!E);
    const EMPTY: bool = "".is_empty();
    assert!(EMPTY);
}
}

impl<T, const N: usize> ListImpl<T, N> {
    const fn as_list(&self) -> &List<T> {
        unsafe { std::mem::transmute(self) }
    }
}

pub static A: ListImpl<i16, 3> = ListImpl {
    len: 3,
    data: [5, 6, 7],
};
pub static A_REF: &'static List<u128> = A.as_list();
pub static A_TAIL_OFFSET: isize = tail_offset(A.int_16());

const fn tail_offset<T>(list: &List<T>) -> isize {
    unsafe { (addr_of!(list.tail) as *const u8).offset_from(list as *const List<T> as *const u8) }
}

fn main() {
    assert_eq!(A_REF.as_slice(), &[5, 6, 7]);
    // Check that interpreter and code generation agree about the position of the tail field.
    assert_eq!(A_TAIL_OFFSET, tail_offset(A_REF));
}
