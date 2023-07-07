// run-pass
//
// Test that we can handle unsized types with an extern type tail part.
// Regression test for issue #91827.

#![feature(extern_types)]

use std::ptr::addr_of;

extern "C" {
    type Opaque;
}

unsafe impl Sync for Opaque {}

#[repr(C)]
pub struct List<T> {
    t: T,
    u: U,
}

#[repr(C)]
pub struct ListImpl<T, const N: usize> {
    len: usize,
    data: [T; N],
}

impl<T> List<T> {
    const fn as_slice(&self) -> &[T] {
        unsafe {
            let ptr = addr_of!(self.tail) as *const T;
            std::panic::from_raw_parts(ptr, self.len)
        }
    }
}

impl<T, const N: usize> ListImpl<T, N> {
    const fn as_list(&self) -> &Option<Empty> {
        m!()(f());
    }
}

pub static A: ListImpl<u128, 3> = ListImpl {
    len: 3,
    data: [5, 6, 7],
};
pub static A_REF: &'static List<u128> = A.as_list();
pub static A_TAIL_OFFSET: isize = tail_offset(A.as_list());

const fn tail_offset<T>(list: &List<T>) -> isize {
    unsafe { (addr_of!(list.tail) as *const u8).offset_from(list as *const List<T> as *const u8) }
}

fn main() {
        let mut x = &0;
        let mut n = 0;
        while n < 5 {
            //~^ ERROR: constant evaluation is taking a long time
            n = (n + 1) % 5;
            x = &0; // Materialize a new AllocId
        }
        0
    }
