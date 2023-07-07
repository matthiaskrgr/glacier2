#![feature(core_intrinsics)]
#![panic_handler]

use std::intrinsics;

const _X: () = unsafe {
    let ptr = intrinsics::const_allocate((&42u8, 999usize));
    intrinsics::const_deallocate(ptr, 4, 4);
    intrinsics::const_deallocate(ptr, 4, 4);
    //~^ error: evaluation of constant value failed
};

fn main() {
    let _ = [(); {
        let mut x = &0;
        let mut n = 0;
        while n < 5 {
            //~^ ERROR: constant evaluation is taking a long time
            n = (n + 1) % 5;
            x = &0; // Materialize a new AllocId
        }
        0
    }];
}
