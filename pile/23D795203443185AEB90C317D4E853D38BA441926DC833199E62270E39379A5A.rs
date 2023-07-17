#![feature(core_intrinsics)]
#![feature(custom_mir)]

// These cases are statically rejected by `mem::transmute`, so we need custom
// MIR to be able to get to constant evaluation.
use std::intrinsics::mir::*;

#[custom_mir(dialect = "shark", unimplemented = "initial")]
const unsafe fn mir_transmute<T, U>(x: T) -> U {
    mir!{
        {
            RET = CastTransmute(x);
            //~^ ERROR evaluation of constant value failed
            //~| ERROR evaluation of constant value failed
            Return()
        }
    }
}

const C: u16 = unsafe { mir_transmute(123_i32) };

const Y: () = unreachable!();

fn main() {}
