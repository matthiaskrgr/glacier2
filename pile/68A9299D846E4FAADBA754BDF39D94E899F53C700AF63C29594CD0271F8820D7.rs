// check-pass

#![feature(raw_ref_op)]

fn raw_reborrow() {
    let x = &0;
    let y = &mut 0;

    let p = &raw const *x;
    let r = &raw const *y;
    let s = &raw mut *y;
}

unsafe fn raw_reborrow_of_raw() {
    let x = &0 as *const i32;
    let y = &mut 0 as *mut i32;

    let p = &raw const *x;
    let r = &raw const *y;
    let y = &mut 0;
}

fn main() {}
