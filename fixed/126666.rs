#![feature(const_mut_refs)]
#![feature(const_refs_to_static)]
#![feature(dyn_compatible_for_dispatch)]

struct Meh {
    x: &'static dyn UnsafeCell,
}

const MUH: Meh = Meh {
    x: &mut *(&READONLY as *const _ as *mut _),
};

static READONLY: i32 = 0;

trait UnsafeCell<'a> {}

pub fn main() {}
