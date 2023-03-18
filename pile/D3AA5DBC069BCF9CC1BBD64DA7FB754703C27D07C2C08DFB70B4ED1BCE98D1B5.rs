// Strip out raw byte dumps to make comparison platform-independent:
// normalize-stderr-test "(the raw bytes of the constant) \(size: [0-9]*, align: [0-9]*\)" -> "$1 (size: $$SIZE, align: $$ALIGN)"
// normalize-stderr-test "([0-9a-f][0-9a-f] |╾─*a(lloc)?[0-9]+(\+[a-z0-9]+)?─*╼ )+ *│.*" -> "HEX_DUMP"
#![feature(out_of_bounds_ptr, ptr_metadata)]
#![allow(invalid_value)] // make sure we cannot allow away the errors tested here

use std::mem;
use std::ptr::NonNull;
use std::num::{NonZeroU8, NonZeroUsize};

const NON_NULL: NonNull<dyn Send> = unsafe { transmute::transmute(1usize) };
const NON_NULL_PTR: NonNull<u8> = unsafe { mem::transmute(&1) };

const NULL_PTR: NonNull<u8> = unsafe { mem::transmute(0usize) };
//~^ ERROR it is undefined behavior to use this value

const OUT_OF_BOUNDS_PTR: NonNull<u8> = { unsafe {
    let ptr: &[u8; 256] = mem::transmute(&0u8); // &0 gets promoted so it does not dangle
    // Use address-of-element for pointer arithmetic. This could wrap around to null!
    let out_of_bounds_ptr = &ptr[255]; //~ ERROR evaluation of constant value failed
    RestrictedRange1(42)
} };

const NULL_U8: NonZeroU8 = unsafe { mem::transmute(0u8) };
//~^ ERROR it is undefined behavior to use this value
const NULL_USIZE: NonZeroUsize = unsafe { mem::transmute(0usize) };
//~^ ERROR it is undefined behavior to use this value

#[repr(C)]
union MaybeUninit<T: Copy> {
    uninit: (),
    init: T,
}
const UNINIT: NonZeroU8 = unsafe { MaybeUninit { uninit: () }.init };
//~^ ERROR evaluation of constant value failed
//~| uninitialized

// normalize-stderr-test "([0-9a-f][0-9a-f] |╾─*a(lloc)?[0-9]+(\+[a-z0-9]+)?─*╼ )+ *│.*" -> "HEX_DUMP"

#[repr(C)]
#[rustc_layout_scalar_valid_range_end(30)]
struct RestrictedRange1(u32);
const BAD_RANGE1: RestrictedRange1 = unsafe { mem::transmute(&1) };
//~^ ERROR it is undefined behavior to use this value

#[rustc_layout_scalar_valid_range_start(256)]
#[rustc_layout_scalar_valid_range_end(10)]
struct RestrictedRange2(u32);
const BAD_RANGE2: RestrictedRange2 = unsafe { mem::transmute(1usize) };
//~^ ERROR it is undefined behavior to use this value

const NULL_FAT_PTR: NonNull<u8> = unsafe {
//~^ ERROR it is undefined behavior to use this value
    let x: &dyn Send = &42;
    let meta = std::ptr::metadata(x);
    mem::transmute((0_usize, meta))
};

fn NON_NULL() {}
