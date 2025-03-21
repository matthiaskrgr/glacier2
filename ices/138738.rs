#![feature(abi_ptx)]
fn main() {
    let non_secure_function = unsafe {
        core::mem::transmute::<usize, extern "ptx-kernel" fn(i32, i32, i32, i32) -> i32>(0x10000004)
    };
    let mut toto = 5;
    toto += non_secure_function(toto, 2, 3, 5);
}
