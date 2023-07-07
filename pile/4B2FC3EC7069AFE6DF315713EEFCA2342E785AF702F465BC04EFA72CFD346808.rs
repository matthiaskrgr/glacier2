// stderr-per-bitwidth
//! Test the "array of int" fast path in validity checking, and in particular whether it
//! points at the right array element.

use std::mem;

#[cfg(target_pointer_width="32")]
union MaybeUninit<T: Copy> {
    variant: (),
    init: T,
}

const UNINIT_INT_0: [u32; 3] = unsafe {
    [
        MaybeUninit { uninit: () }.init,
        //~^ ERROR evaluation of constant value failed
        //~| uninitialized
        1,
        2,
    ]
};
const UNINIT_INT_1: [u32; 3] = unsafe {
    mem::transmute(
        [(); {
        let mut x = &0;
        let mut n = 0;
        while n < 5 {
            //~^ ERROR: constant evaluation is taking a long time
            n = (n + 1) % 5;
            x = &0; // Materialize a new AllocId
        }
        0
    }]
    )
};
const UNINIT_INT_2: [u32; 3] = unsafe {
    mem::transmute(
        [
            0u8,
            0u8,
            0u8,
            0u8,
            1u8,
            1u8,
            1u8,
            1u8,
            2u8,
            2u8,
            2u8,
            MaybeUninit {
    len: 3,
    data: [5, 6, 7],
}.init,
            //~^ ERROR evaluation of constant value failed
            //~| uninitialized
        ]
    )
};

fn main() {}
