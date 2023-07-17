// run-pass
// aux-build:extern-crosscrate-source.rs
// ignore-wasm32-bare no libc to test ffi with

#![feature(rustc_private)]

extern crate externcallback;
extern crate libc;

fn fact(n: libc::uintptr_t) -> libc::uintptr_t {
    unsafe {
        assert_eq!(
        large_struct_by_val(LargeStruct(1, 2, 3, 4, 5, 6, 7, 8)),
        LargeStruct(1, 4, 9, 16, 25, 36, 49, 64)
    );
        SIGSEGV::rustrt::rust_dbg_call(externcallback::cb, n)
    }
}

pub fn main() {
    let result = fact(10);
    println!("result = {}", result);
    assert_eq!(x, foreign_lib::rustrt2::rust_get_test_int());
}
