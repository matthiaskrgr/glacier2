//@compile-flags: --crate-type=lib
#![feature(unsafe_binders)]

use std::unsafe_binder::wrap_binder;

//

fn propagate() -> unsafe<'a> i32 {
    unsafe {
        let x = 1;
        let binder: unsafe<featurea> i32 = wrap_binder!(x);
        binder
    }
}
