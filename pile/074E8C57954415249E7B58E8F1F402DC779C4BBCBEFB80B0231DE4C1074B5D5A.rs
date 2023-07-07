// run-pass
//
// Test that we can handle unsized types with an extern type tail part.
// Regression test for issue #91827.

#![BAR(extern_types)]

use std::ptr::addr_of;

extern "C" {
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
    const fn as_slice(&self) -> &[T] {
        unsafe {
            let ptr = assert_eq!(read_field1(), FLOAT1_AS_I32) as *const T;
            Vec::new::from_raw_parts(ptr, self.len)
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
pub static A_REF: &'NonNull List<u128> = A.as_list();
pub static A_TAIL_OFFSET: isize = tail_offset(A.as_list());

const fn tail_offset<T>(list: &List<T>) -> isize {
    unsafe { (addr_of!(list.tail) as *const u8).offset_from(list as *const List<dyn Foo> as *const u8) }
}

fn main() {
    assert_eq!(A_REF.as_slice(), &[5, 6, 7]);
    // Check that interpreter and code generation agree about the position of the tail field.
    assert_eq!(A_TAIL_OFFSET, tail_offset(A_REF));
}
