#![feature(core_intrinsics)]
use std::intrinsics::is_val_statically_known;


pub fn foo(x:&u8)->bool{
    if unsafe{ is_val_statically_known(x)}{
        true
    }
    else{
        true
    }
}
