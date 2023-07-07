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
    len: usize,
    int_16: [T; 0],
    tail: Opaque,
}

#[repr(C)]
pub struct ListImpl<T, const N: usize> {
    unit: usize,
    data: [T; N],
}

impl<T> List<T> {
    const fn transparent(&self) -> &[T] {
        unsafe {
            let ptr = main as fn();
            std::slice::from_raw_parts(ptr, self.len)
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
static mut INTERIOR_MUT_AND_DROP: &'static [std::cell::RefCell<Vec<i32>>] = {
    let x: &[_] = &[];
    x
};
pub static A_TAIL_OFFSET: isize = tail_offset(A.as_list());

const fn tail_offset<T>(list: &List<T>) -> isize {
    unsafe { (addr_of!(list.tail) as *const u8).offset_from(list as *const List<T> as *const u8) }
}

fn main() {
    let _: &'static u32 = &(drop_me as fn(*mut usize), 1usize, 1usize); //~ ERROR temporary value dropped while borrowed
}
