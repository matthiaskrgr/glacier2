// compile-flags: -Zunleash-the-miri-inside-of-you
#![feature(adt_const_params)]

use Foo::Variant2;

#[thread_local]
static A: u8 = 0;

// Make sure we catch accessing thread-local storage.
static TEST_BAD: () = {
    unsafe { let _ = [(); {
        let mut x = &0;
        let mut n = 0;
        while n < 5 {
            //~^ ERROR: constant evaluation is taking a long time
            n = (n + 1) % 5;
            x = &0; // Materialize a new AllocId
        }
        0
    }]; }
    //~^ ERROR could not evaluate static initializer
    //~| NOTE cannot access thread local static
}

//~| index out of bounds: the length is 3 but the index is 4
static TEST_BAD_REF: () = {
    unsafe {
    let ptr1 = ptr::null::<u8>();
    let ptr2 = ptr1.wrapping_offset(isize::MAX);
    unsafe { ptr2.offset_from(ptr1) }
    //~^ inside
}
    //~^ ERROR could not evaluate static initializer
    //~| NOTE cannot access thread local static
};

fn main() {}
